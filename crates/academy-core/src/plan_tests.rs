use super::*;
use bigdecimal::BigDecimal;

fn oracle(customers: &QueryResult, orders: &QueryResult) -> Vec<QueryResult> {
    let paid: Vec<_> = orders
        .rows
        .iter()
        .filter(|r| r[2].as_deref() == Some("paid"))
        .collect();
    let amount = |r: &Vec<Option<String>>| r[3].as_ref().map(|n| BigDecimal::from_str(n).unwrap());
    let first = paid
        .iter()
        .filter(|r| amount(r).is_some_and(|n| n >= BigDecimal::from(100)))
        .map(|r| vec![r[0].clone(), r[3].clone()])
        .collect();
    let second = orders
        .rows
        .iter()
        .filter(|r| r[1].as_deref() == Some("201"))
        .map(|r| vec![r[0].clone(), r[2].clone(), r[3].clone()])
        .collect();
    let third = customers
        .rows
        .iter()
        .filter(|c| !orders.rows.iter().any(|r| r[1] == c[0]))
        .cloned()
        .collect();
    let daily: Vec<_> = paid
        .iter()
        .filter(|r| &r[4].as_deref().unwrap()[..10] == "2026-02-02")
        .collect();
    let summary = |rows: &[&Vec<Option<String>>]| {
        let known: Vec<_> = rows.iter().filter_map(|r| amount(r)).collect();
        vec![
            Some(rows.len().to_string()),
            Some(known.len().to_string()),
            Some(known.into_iter().sum::<BigDecimal>().to_string()),
        ]
    };
    let fourth = vec![summary(&daily.into_iter().copied().collect::<Vec<_>>())];
    let fifth = customers
        .rows
        .iter()
        .map(|c| {
            let matches: Vec<_> = paid.iter().filter(|r| r[1] == c[0]).copied().collect();
            let measures = summary(&matches);
            vec![
                c[0].clone(),
                c[1].clone(),
                measures[0].clone(),
                measures[2].clone(),
            ]
        })
        .collect();
    let mut ranked = paid;
    ranked.sort_by(|a, b| {
        amount(b).cmp(&amount(a)).then_with(|| {
            a[0].as_deref()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                .cmp(&b[0].as_deref().unwrap().parse::<i32>().unwrap())
        })
    });
    assert!(
        ranked[..5].windows(2).any(|p| amount(p[0]) == amount(p[1])),
        "Ranking must exercise ties"
    );
    let sixth = ranked
        .into_iter()
        .take(5)
        .map(|r| vec![r[0].clone(), r[3].clone()])
        .collect();
    [
        (vec![23, 1700], first),
        (vec![23, 25, 1700], second),
        (vec![23, 25], third),
        (vec![20, 20, 1700], fourth),
        (vec![23, 25, 20, 1700], fifth),
        (vec![23, 1700], sixth),
    ]
    .into_iter()
    .map(|(column_types, rows)| QueryResult {
        column_types,
        rows,
        ..Default::default()
    })
    .collect()
}

#[tokio::test]
async fn first_plan_chapter_has_six_authored_measured_queries() {
    let home = tempfile::tempdir().unwrap();
    let runtime = std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
    let mut academy = Academy::start(home.path(), &runtime).await.unwrap();
    let chapter = academy
        .chapters
        .get(5)
        .expect("Chapter six must be authored");
    assert!(!chapter.published && chapter.exercises.len() == 6);
    assert!(chapter.exercises.iter().all(|e| e.require_analyze));
    let exercises = chapter.exercises.clone();
    for chapter in &mut academy.chapters {
        chapter.published = true;
        for e in &chapter.exercises {
            academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
            academy.store.pass(&e.id, &e.revision).unwrap();
        }
    }
    for seed in 0..3 {
        let database = format!("oracle_{seed}");
        let role = format!("oracle_student_{seed}");
        academy
            .cluster
            .create_lab(&database, &role, Fixture::Commerce, seed)
            .await
            .unwrap();
        let learner = academy.cluster.connect(&database, &role).await.unwrap();
        let customers = execute(
            &learner,
            "SELECT id,name FROM customers ORDER BY id",
            "run",
            1000,
        )
        .await
        .unwrap();
        let orders = execute(
            &learner,
            "SELECT id,customer_id,status,total,ordered_at FROM orders ORDER BY id",
            "run",
            1000,
        )
        .await
        .unwrap();
        for (e, expected) in exercises.iter().zip(oracle(&customers, &orders)) {
            let actual = execute(&learner, &e.solution_sql, "run", 1000)
                .await
                .unwrap();
            assert!(
                actual.error.is_none() && !actual.truncated,
                "{} seed {seed}",
                e.id
            );
            assert_eq!(
                normalized(&actual),
                normalized(&expected),
                "{} seed {seed}",
                e.id
            );
            let measured = execute(&learner, &e.solution_sql, "analyze", 1000)
                .await
                .unwrap();
            assert!(measured.error.is_none());
            let root = &measured.plan.as_ref().unwrap()[0]["Plan"];
            assert_eq!(
                root["Actual Rows"].as_f64(),
                Some(expected.rows.len() as f64)
            );
            assert_eq!(root["Actual Loops"].as_f64(), Some(1.0));
        }
        academy.cluster.drop_lab(&database, &role).await.unwrap();
    }
    for e in &exercises {
        academy
            .run(request(&e.id, &e.solution_sql, "analyze"))
            .await
            .unwrap();
        assert!(
            academy
                .check(request(&e.id, &e.solution_sql, "run"))
                .await
                .unwrap()
                .passed
        );
    }
    academy.shutdown().await.unwrap();
}

#[tokio::test]
async fn cancelled_analysis_does_not_supply_measurement_evidence() {
    let home = tempfile::tempdir().unwrap();
    let runtime = std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
    let mut academy = Academy::start(home.path(), &runtime).await.unwrap();
    require_measurement(&mut academy);
    let academy = std::sync::Arc::new(academy);
    let input = request("02-01", "SELECT pg_sleep(20)", "analyze");
    let run_id = input.run_id.clone();
    let runner = academy.clone();
    let task = tokio::spawn(async move { runner.run(input).await.unwrap() });
    let admin = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    let mut sleeping = false;
    for _ in 0..200 {
        sleeping = admin.query_one("SELECT EXISTS(SELECT FROM pg_stat_activity WHERE datname='lab_02_01' AND wait_event='PgSleep')",&[]).await.unwrap().get(0);
        if sleeping {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert!(sleeping, "Cancel a query that actually started executing");
    assert!(academy.cancel(&run_id).await.unwrap());
    assert_eq!(
        task.await.unwrap().error.unwrap().code.as_deref(),
        Some("57014")
    );
    assert_eq!(
        academy.progress().await.unwrap()["02-01"]["analyzed"],
        false
    );
    assert!(
        !academy
            .check(request("02-01", "SELECT pg_sleep(20)", "run"))
            .await
            .unwrap()
            .passed
    );
    academy.shutdown().await.unwrap();
}

fn require_measurement(academy: &mut Academy) {
    for chapter in &mut academy.chapters[..2] {
        chapter.published = true;
    }
    for e in &academy.chapters[0].exercises {
        academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
        academy.store.pass(&e.id, &e.revision).unwrap();
    }
    let mut exercise = serde_json::to_value(&academy.chapters[1].exercises[0]).unwrap();
    exercise["solution_sql"] = json!(academy.chapters[1].exercises[0].solution_sql);
    exercise["explanation"] = json!(academy.chapters[1].exercises[0].explanation);
    exercise["require_analyze"] = json!(true);
    academy.chapters[1].exercises[0] = serde_json::from_value(exercise).unwrap();
}
fn request(id: &str, sql: &str, mode: &str) -> RunRequest {
    RunRequest {
        exercise_id: id.into(),
        sql: sql.into(),
        mode: mode.into(),
        run_id: uuid::Uuid::new_v4().to_string(),
    }
}

#[tokio::test]
async fn measured_evidence_is_required_and_bound_to_current_sql_revision_and_lab() {
    let home = tempfile::tempdir().unwrap();
    let runtime = std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
    let mut academy = Academy::start(home.path(), &runtime).await.unwrap();
    require_measurement(&mut academy);
    let e = academy.chapters[1].exercises[0].clone();
    let sql = e.solution_sql.clone();
    assert!(
        !academy
            .check(request(&e.id, &sql, "run"))
            .await
            .unwrap()
            .passed,
        "Correct results alone must not satisfy a measured-plan exercise"
    );
    assert!(
        academy.solution(&e.id).unwrap().is_some(),
        "Check still unlocks an answer before Analyze"
    );
    assert!(!academy.progress().await.unwrap()[&e.id]["analyzed"]
        .as_bool()
        .unwrap());
    academy.run(request(&e.id, &sql, "explain")).await.unwrap();
    assert!(
        !academy
            .check(request(&e.id, &sql, "run"))
            .await
            .unwrap()
            .passed,
        "Estimates are not measurements"
    );
    academy
        .run(request(&e.id, "SELEC", "analyze"))
        .await
        .unwrap();
    assert!(
        !academy
            .check(request(&e.id, &sql, "run"))
            .await
            .unwrap()
            .passed,
        "A failed Analyze is not evidence"
    );
    let result = academy.run(request(&e.id, &sql, "analyze")).await.unwrap();
    assert!(
        result.error.is_none()
            && result.plan.as_ref().unwrap()[0]["Plan"]["Actual Rows"].is_number()
    );
    assert!(
        academy
            .check(request(&e.id, &sql, "run"))
            .await
            .unwrap()
            .passed
    );
    let alternative = format!("{sql} -- same result, independently measured SQL");
    academy.save(&e.id, &alternative).unwrap();
    assert_eq!(academy.progress().await.unwrap()[&e.id]["analyzed"], false);
    assert!(
        !academy
            .check(request(&e.id, &alternative, "run"))
            .await
            .unwrap()
            .passed
    );
    academy
        .run(request(&e.id, &alternative, "analyze"))
        .await
        .unwrap();
    assert!(
        academy
            .check(request(&e.id, &alternative, "run"))
            .await
            .unwrap()
            .passed,
        "Equivalent spellings pass after their own measurement"
    );
    academy.chapters[1].exercises[0].revision = "new-measurement-revision".into();
    assert!(
        !academy
            .check(request(&e.id, &alternative, "run"))
            .await
            .unwrap()
            .passed
    );
    academy.chapters[1].exercises[0].revision = e.revision.clone();
    academy.shutdown().await.unwrap();
    drop(academy);
    let mut reopened = Academy::start(home.path(), &runtime).await.unwrap();
    require_measurement(&mut reopened);
    assert_eq!(
        reopened.progress().await.unwrap()[&e.id]["analyzed"],
        true,
        "Evidence survives reopening"
    );
    assert!(
        reopened
            .check(request(&e.id, &alternative, "run"))
            .await
            .unwrap()
            .passed
    );
    reopened.reset(&e.id).await.unwrap();
    assert_eq!(reopened.progress().await.unwrap()[&e.id]["analyzed"], false);
    assert_eq!(
        reopened.store.script(&e.id).unwrap().as_deref(),
        Some(alternative.as_str())
    );
    assert!(
        reopened.store.passed(&e.id, &e.revision).unwrap(),
        "Reset preserves the earned pass"
    );
    assert!(
        !reopened
            .check(request(&e.id, &alternative, "run"))
            .await
            .unwrap()
            .passed
    );
    reopened.shutdown().await.unwrap();
}

export type Exercise={id:string;title:string;prompt:string;starter:string;ordered:boolean;revision:string};
export type Chapter={id:string;published:boolean;title:string;subtitle:string;objectives:string[];exercises:Exercise[];transcript:string[]};
export type Course={chapter:Chapter;chapters:Chapter[];planned_chapters:number;planned_exercises:number;available_chapters:number;server_version:string;media_directory?:string};
export type Progress=Record<string,{unlocked:boolean;passed:boolean;sql:string|null}>;
export type QueryResult={columns:string[];column_types:number[];rows:(string|null)[][];plan:Plan[]|null;error:{message:string;code:string|null;position:number|null}|null;truncated:boolean;elapsed_ms:number};
export type Plan={Plan:PlanNode;'Planning Time'?:number;'Execution Time'?:number};
export type PlanNode={'Node Type':string;Plans?:PlanNode[];[key:string]:unknown};
export type CheckResult={passed:boolean;message:string;variants_passed:number;variants_total:number};

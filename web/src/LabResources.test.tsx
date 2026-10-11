import {afterEach,expect,test,vi} from 'vitest';
import {act,cleanup,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {LabResources} from './LabResources';
import {call} from './api';
vi.mock('./api',()=>({call:vi.fn()}));
afterEach(()=>{cleanup();vi.clearAllMocks();});
const snapshot={database_bytes:1048576,available_disk_bytes:1073741824,minimum_free_disk_bytes:536870912,work_mem:'4MB',shared_buffers:'64MB',temp_file_limit:'128MB'};
function toggle(open:boolean){const detail=screen.getByText('Lab resources').closest('details')!;detail.open=open;fireEvent(detail,new Event('toggle'));}
test('resources fetch only when opened and distinguish actual storage from setting allowances',async()=>{
 vi.mocked(call).mockResolvedValue(snapshot);
 render(<LabResources id="01-01"/>);
 expect(call).not.toHaveBeenCalled();
 toggle(true);
 expect(await screen.findByText('1.0 MiB')).toBeTruthy();
 expect(call).toHaveBeenCalledWith('resources',{id:'01-01'});
 for(const text of ['4MB','64MB','128MB','Database storage excludes WAL, logs and temporary files.','Work memory is per operation; spill allowance is per PostgreSQL process.'])expect(screen.getByText(text)).toBeTruthy();
 toggle(false);
 expect(screen.queryByText('1.0 MiB')).toBeNull();
});
test('resource errors are recoverable on reopening',async()=>{
 vi.mocked(call).mockRejectedValueOnce(new Error('Disk measurement unavailable')).mockResolvedValue(snapshot);
 render(<LabResources id="01-01"/>);toggle(true);
 expect((await screen.findByRole('alert')).textContent).toContain('Disk measurement unavailable');
 toggle(false);toggle(true);
 expect(await screen.findByText('1.0 MiB')).toBeTruthy();
 expect(screen.queryByRole('alert')).toBeNull();
});
test('a late snapshot from a collapsed request cannot replace the reopened measurement',async()=>{
 let finish!:(v:typeof snapshot)=>void;
 vi.mocked(call).mockImplementationOnce(()=>new Promise(resolve=>{finish=resolve;})).mockResolvedValue({...snapshot,database_bytes:2097152});
 render(<LabResources id="01-01"/>);toggle(true);
 await waitFor(()=>expect(call).toHaveBeenCalledTimes(1));
 expect(screen.getByText('Reading lab resources…')).toBeTruthy();
 toggle(false);toggle(true);
 expect(await screen.findByText('2.0 MiB')).toBeTruthy();
 await act(async()=>finish(snapshot));
 expect(screen.getByText('2.0 MiB')).toBeTruthy();
 expect(screen.queryByText('1.0 MiB')).toBeNull();
});
test('switching exercises starts a closed disclosure and ignores the old exercise response',async()=>{
 let finish!:(v:typeof snapshot)=>void;
 vi.mocked(call).mockImplementationOnce(()=>new Promise(resolve=>{finish=resolve;})).mockResolvedValue({...snapshot,database_bytes:2097152});
 const view=render(<LabResources key="01-01" id="01-01"/>);toggle(true);
 await waitFor(()=>expect(call).toHaveBeenCalledTimes(1));
 view.rerender(<LabResources key="01-02" id="01-02"/>);
 expect(screen.getByText('Lab resources').closest('details')!.open).toBe(false);
 toggle(true);
 expect(await screen.findByText('2.0 MiB')).toBeTruthy();
 expect(call).toHaveBeenLastCalledWith('resources',{id:'01-02'});
 await act(async()=>finish(snapshot));
 expect(screen.queryByText('1.0 MiB')).toBeNull();
});

use multitasking::scheduler;
use sys::time;

pub fn init() {
    for _i in 0..2 {
        scheduler::spawn(foo);
    }
}

fn foo() {
    let mut counter = 0;
    system_log!(">>>    task_{}: started", scheduler::current_task_id());

    for _i in 0..3 {
        counter += 1;
        system_log!(
            ">>>    task_{}: hello! counter={}",
            scheduler::current_task_id(),
            counter,
        );

        time::stupid_sleep();
    }

    system_log!(
        ">>>    task_{}: completed, stopping...",
        scheduler::current_task_id()
    );
}

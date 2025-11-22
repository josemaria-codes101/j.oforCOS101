fn main() {

    println!("===================================================================");
    println!("               PUBLIC SERVICE APS LEVEL CHECKER                    ");
    println!("===================================================================");
    println!("Welcome!");
    println!("--------");

    let positions: Vec<(&str, &str, &str)> = vec![
    // for APS 1-2
    ("APS 1-2", "Office Administator", "Intern"),
    ("APS 1-2", "Academic", "-"),
    ("APS 1-2", "Lawyer", "Paralegal"),
    ("APS 1-2", "Teacher", "Placement"),

    // for APS 3-5
    ("APS 3-5", "Office Administator", "Administator"),
    ("APS 3-5", "Academic", "Research Assistant"),
    ("APS 3-5", "Lawyer", "Junior Associate"),
    ("APS 3-5", "Teacher", "Classroom Teacher"),

    // for APS 5-8
    ("APS 5-8", "Office Administator", "Senior Administator"),
    ("APS 5-8", "Academic", "PhD Candidate"),
    ("APS 5-8", "Lawyer", "Associate"),
    ("APS 5-8", "Teacher", "Snr Teacher"),

    // for EL1 8-10
    ("EL1 8-10", "Office Administator", "Office Manager"),
    ("EL1 8-10", "Academic", "Post-Doc Researcher"),
    ("EL1 8-10", "Lawyer", "Senior Associate 1-2"),
    ("EL1 8-10", "Teacher", "Leading Teacher"),

    // for EL2 10-13
    ("EL2 10-13", "Office Administator", "Director"),
    ("EL2 10-13", "Academic", "Senior Lecturer"),
    ("EL2 10-13", "Lawyer", "Senior Associate 3-4"),
    ("EL2 10-13", "Teacher", "Deputy Principal"),

    // for SES
    ("SES", "Office Administator", "CEO"),
    ("SES", "Academic", "Dean"),
    ("SES", "Lawyer", "Partner"),
    ("SES", "Teacher", "Principal")];

    let mut job = String::new();
    println!("Enter job field (Office Administator / Academic / Lawyer / Teacher): ");
    std::io::stdin().read_line(&mut job).expect("Failed to read input");
    job = job.trim().to_string();

    let mut valid_job = false;
    for (_,job_field,_) in &positions {
        if job.eq_ignore_ascii_case(job_field) {
            valid_job = true;
            break;
        }
    }
    if !valid_job {
        println!("Job invalid!, Please try again");
        return;
    }

    let mut sub_job = String::new();
    println!("Enter sub-job title (eg. Intern, Research Assistant, PhD Candidate, etc): ");
    std::io::stdin().read_line(&mut sub_job).expect("Failed to read input");
    sub_job = sub_job.trim().to_string();

    for (aps_level, job_field, job_title) in &positions {
        if job.eq_ignore_ascii_case(job_field) && sub_job.eq_ignore_ascii_case(job_title) {
            println!("Staff Level: {}", aps_level);
            return;
        }

    }
    println!("No staff level found, Please try again");
}

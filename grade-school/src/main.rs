use grade_school as school;

fn main() {
    println!("Grade school");
    let mut school = school::School::new();
    school.add(1, "Picard");
    school.add(2, "Riker");
    school.add(3, "Data");
    school.add(3, "Geordi");
    school.add(3, "Brackley");

    
    school.grade(1);
    school.grades();
}
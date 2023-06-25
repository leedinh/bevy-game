use bevy::prelude::*;

fn main() {
        App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}

pub struct  PeoplePlugin ;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_system(print_names)
        .add_system(people_with_jobs)
        .add_system(people_does_job);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(
       ( Person {
        name: "Dinh".to_string(),
    },
    Employed {
        job: Job::Lawyer,
    })
);
    commands.spawn((Person {
        name: "Thi".to_string(),
    },
    Employed {
        job: Job::Lawyer,
    })
);
    commands.spawn((Person {
        name: "GG".to_string(),
    },
    Employed {
        job: Job::Lawyer,
    })
);
}


pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {:?}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job", person.name);
    }
}

pub fn people_does_job(person_query: Query<(&Person,&Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Lawyer => "Lawyer",
            Job::Doctor => "Doctor",
            Job::Engineer => "Engineer",
        };
        println!("{0} is a {1}", person.name, job_name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}


#[derive(Component)]
pub struct  Employed {
    pub job: Job,
}

pub enum Job {
    Doctor,
    Engineer,
    Lawyer,
}
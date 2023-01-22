mod cli;
mod constants;
mod template;
mod utils;

mod cli2; // ! remove this later

// ! touch nothing below

fn main() {
    cli::run();
    // cli2::run(); // ! remove this

    // utils::new_file("name");
}

//immediate
// a spinning backpack svg with the title WAO

/* // ! IDEAS
 todo -> ability to publish an xnft
 todo -> ability to use different templates(i.e create more templates)
 todo (think) -> what to do if creating a new project does not complete because of spurious network errors
 todo -> what to do when the folder/file exists

*/

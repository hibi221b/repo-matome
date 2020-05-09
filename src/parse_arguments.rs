use clap::App;

pub fn parse_arg<'a>() -> clap::ArgMatches<'a> {
    App::new(crate_name!())
        .version(crate_version!())      
        .author(crate_authors!())       
        .about(crate_description!())
        .get_matches()
}
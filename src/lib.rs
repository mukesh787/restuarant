
mod front_of_house{
    // fn serve_order(){}
    pub mod hosting{
        pub fn add_to_waitlist(){
            // super::serve_order();   //for super keyword;
        }
        
    }
}

mod back_of_house{
    pub struct Breakfast{
        pub toast:String,
        pub seasonal_fruit:String,
    }
    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast { toast:String::from(toast), seasonal_fruit:String::from("peaches") }
        }
    }
}

// pub fn restaurant(){
//     // crate::front_of_house::hosting::add_to_waitlist();

//     // front_of_house::hosting::add_to_waitlist();

//     // let mut meal=back_of_house::Breakfast::summer("rice");
//     let mut meal=back_of_house::Breakfast{
//         toast:String::from("Rye"),
//         seasonal_fruit:String::from("peaxhes")
//     };
//     meal.toast=String::from("wheat");
// }
use crate::front_of_house::hosting;
pub fn restaurant(){
    hosting::add_to_waitlist();
}
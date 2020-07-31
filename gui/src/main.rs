use gui::{SelectBox, Button, Screen};

fn main()
{
    let screen = Screen
    {
        components: vec![
            Box::new(Button
            {
                height: 30,
                width: 30,
                label: String::from("Scream inside your heart")
            }),
            Box::new(SelectBox
            {
                height: 20,
                width: 45,
                options: vec![String::from("Run"), String::from("Ride"), String::from("HIIT")]
            })
        ]
    };

    screen.run();
}
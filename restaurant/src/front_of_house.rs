pub mod hosting;

pub mod serving
{
    fn take_order()
    {
        println!("Taking order");
    }

    pub fn serve_order()
    {
        println!("Serving order");
    }

    fn take_payment()
    {
        println!("Taking payment and probably tip");
    }
}


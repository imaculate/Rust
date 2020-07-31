trait State 
{
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str
    {
        ""
    }
}

struct Post
{
    content: String,
    state: Option<Box<dyn State>>
}

impl Post
{
    pub fn new() -> Post
    {
        Post
        {
            content: String::from(""),
            state: Some(Box::new(Draft{}))
        }
    }

    pub fn content(&self) -> &str
    {
        // why as ref? To borrow a ref to state, deref coercion allows calling content on box directly
        // we can call unwrap because state always has Some(value)
        self.state.as_ref().unwrap().content(self)
    }

    pub fn add_to_text(&mut self, text: &str)
    {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self)
    {
        // take, sets state to None and returns previous value, ensures struct Field always has value
        if let Some(s) = self.state.take()
        {
            self.state = Some(s.request_review());
        }
        
    }

    pub fn approve(&mut self)
    {
        // take, sets state to None and returns previous value, ensures struct Field always has value
        if let Some(s) = self.state.take()
        {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self)
    {
        // take, sets state to None and returns previous value, ensures struct Field always has value
        if let Some(s) = self.state.take()
        {
            self.state = Some(s.reject());
        }
    }
}

struct Draft {}

impl State for Draft
{
    fn request_review(self: Box<Self>) -> Box<dyn State>
    {
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State>
    {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State>
    {
        self
    }
}

struct PendingReview {}

impl State for PendingReview
{
    fn request_review(self: Box<Self>) -> Box<dyn State>
    {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State>
    {
        Box::new(Published{})
    }

    fn reject(self: Box<Self>) -> Box<dyn State>
    {
        Box::new(Draft{})
    }
}

struct Published {}

impl State for Published
{
    fn request_review(self: Box<Self>) -> Box<dyn State>
    {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State>
    {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State>
    {
        Box::new(Draft{})
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str
    {
        &post.content
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn state_and_content()
    {
        let mut post = Post::new();
        assert_eq!("", post.content());

        post.add_to_text("I had burritos or breakfast");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I had burritos or breakfast", post.content());

        post.reject();
        assert_eq!("", post.content());
    }
}

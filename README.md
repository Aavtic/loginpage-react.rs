# Login React Page With Rust(Axum) in Backend.

This is a simple Login page with it's frontend using [React.js](https://react.dev/) and it's Backend using Rust's [Axum](https://docs.rs/axum/latest/axum/) Framework.
This project uses [MongoDB](https://www.mongodb.com/) as it's backend to store Users Information.

This is a template for a login page which handles all these login scenarios
- User Account Creation.
- User Account Login.
Session Management
- Upon a successful login a unique session key is provided to the client which is stored in the browser of the client
    - This session key has an expiry date of 3 days.
    - This means user can stay logged in for three days after loggin in once.
- The authenticity of the user is checked once everytime the user loads up his user page.
- If by anychace the cookie(sessionkey) is malformed the user will automatically be redirected to the login page.

## Live Demo
If you want a demo of the website you can visit [this](https://aavtic.github.io/loginpage-react.rs/) link to view a live demo of the 
same application but in a static way. So you can get a taste of it.

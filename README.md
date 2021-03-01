
# Rust Web Programming

<a href="https://www.packtpub.com/in/web-development/rust-web-programming?utm_source=github&utm_medium=repository&utm_campaign=9781786461629"><img src="https://www.packtpub.com/media/catalog/product/cache/4cdce5a811acc0d2926d7f857dceb83b/9/7/9781800560819-original_23.png" alt="Rust Web Programming" height="256px" align="right"></a>


This is the code repository for [Rust Web Programming](https://www.packtpub.com/in/web-development/rust-web-programming?utm_source=github&utm_medium=repository&utm_campaign=9781786461629), published by Packt.

## Setup 

In order to run the code in this github, you're going to have to install Rust via the following link:

```
https://www.rust-lang.org/tools/install
```

## Running Code 

The code has to be compiled. In order to do this, we navigate to the file we want to run. For this example, we're using ```strings.rs``` in chapter_one. Once we've navigated there, we compile the file using the ```rustc``` command:

```
rustc strings.rs 
```

On Windows, we then run the exe file:

```
.\strings.exe
```

For Linux and Mac, we run using the following:

```
./strings
```

## JSON Web Token
Please note, Chapter 11, Understanding Rocket Web Framework, and the Appendix A, both utlize a JSON web token. However, the token is not created in the chapter and appendix. Below
is a token for a user with the ID of one. This can be copied and pasted for use in the exercises:

```
eyJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjo2fQ.uVo7u877IT2GEMpB_gxVtxhMAYAJD8W_XiUoNvR7_iM
```


**A hands-on guide to developing fast and secure web apps with the Rust programming language**

## What is this book about?
This book on web programming with Rust is for web developers who have programmed in traditional languages such as Python, Ruby, JavaScript, and Java and are looking to develop high-performance web applications with Rust. Although no prior experience with Rust is necessary, a solid understanding of web development principles and basic knowledge of HTML, CSS, and JavaScript are required if you want to get the most out of this book.

This book covers the following exciting features:
Structure scalable web apps in Rust in Rocket, Actix Web, and Warp
Apply data persistence for your web apps using PostgreSQL
Build login, JWT, and config modules for your web apps
Serve HTML, CSS, and JavaScript from the Actix Web server
Build unit tests and functional API tests in Postman and Newman
Deploy the Rust app with NGINX and Docker onto an AWS EC2 instance

If you feel this book is for you, get your [copy](https://www.amazon.com/dp/1800560818) today!

<a href="https://www.packtpub.com/?utm_source=github&utm_medium=banner&utm_campaign=GitHubBanner"><img src="https://raw.githubusercontent.com/PacktPublishing/GitHub/master/GitHub.png" 
alt="https://www.packtpub.com/" border="5" /></a>

## Instructions and Navigations
All of the code is organized into folders. For example, Chapter02.

The code will look like the following:
```
pretty_env_logger::init();
let log = warp::log("to_do::api");
```

**Following is what you need for this book:**
This book on web programming with Rust is for web developers who have programmed in traditional languages such as Python, Ruby, JavaScript, and Java and are looking to develop high-performance web applications with Rust. Although no prior experience with Rust is necessary, a solid understanding of web development principles and basic knowledge of HTML, CSS, and JavaScript are required if you want to get the most out of this book.

## Code in Action
Please visit the following link to check the CiA videos: http://bit.ly/3jULCrw

With the following software and hardware list you can run all code files present in the book (Chapter 1-11).
### Software and Hardware List
| No | Software required | OS required |
| -------- | ------------------------------------ | ----------------------------------- |
| 1 | Rust | Windows, Mac OS X, and Linux (Any) |
| 2 | Docker | Windows, Mac OS X, and Linux (Any) |
| 3 | Docker-compose | Windows, Mac OS X, and Linux (Any) |
| 4 | Postman | Windows, Mac OS X, and Linux (Any) |


We also provide a PDF file that has color images of the screenshots/diagrams used in this book. [Click here to download it](https://static.packt-cdn.com/downloads/9781800560819_ColorImages.pdf).


### Related products
* Creative Projects for Rust Programmers [[Packt]](https://www.packtpub.com/product/creative-projects-for-rust-programmers/9781789346220?utm_source=github&utm_medium=repository&utm_campaign=9781789346220) [[Amazon]](https://www.amazon.com/dp/1789346223)

* Practical System Programming for Rust Developers [[Packt]](https://www.packtpub.com/product/practical-system-programming-for-rust-developers/9781800560963?utm_source=github&utm_medium=repository&utm_campaign=9781800560963) [[Amazon]](https://www.amazon.com/dp/B08MBCQ5L1)


## Get to Know the Author
**Maxwell Flitton** is a Software Engineer who works at a financial tech company called Monolith AI. In 2011, Maxwell achieved his Bachelor of Science in Nursing degree from the University of Lincoln, UK. While working 12 hours shifts in the A and E departments of hospitals, Maxwell obtained another degree in Physics from The Open University in the UK and then moved one to another milestone, with a Postgrad Diploma in Physics and Engineering in Medicine from UCL in London. He developed an open source machine learning deployment software called DeployML which can be downloaded via pip and occasionally he teaches computational medicine at Imperial College London every now and then.

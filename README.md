# Rust-for-Web-Developers
Rust for Web Developers, published by Packt

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
Chapter 11 and the appendix utlize a JSON web token. However, the token is not created in the chapter and appendix. Below
is a token for a user with the id of one. This can be copied and pasted for use in the exercises:

```
eyJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjo2fQ.uVo7u877IT2GEMpB_gxVtxhMAYAJD8W_XiUoNvR7_iM
```

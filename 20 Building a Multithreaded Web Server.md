经过漫长的旅程，我们终于快到了书的终点。这章中，我将会创建一个项目来展示我们曾经学过的知识，并且也可以回顾我们之前学过的知识

下面是我们创建web服务器的计划

1. 学习一些关于TCP和HTTP的知识。
2. 在socket上监听TCP的连接。
3. 解析少量的HTTP请求。
4. 创建一个适当的 response
5. 用线程池来提供吞吐量。

在实现之前，我们需要说一个细节，我们实现的方法不是最好的实现web服务器的方法。在 crate.io中有很多产品级的库，可以提供更多完整的web服务以及线程池。

因为Rust是个系统编程语言呢，我们可以达到更低的抽象级别，而且是别的编程语言达不到的级别。我们将会手续底层的HTTP服务器，还有线程池，这样你可以学习到总体的思想核技术，在以后你可能会用到这些。
# 1 Building a Single-Threaded Web Server

## 1.1 Listening to the TCP Connection

## 1.2 Reading the Request



## 1.3 A Closer Look at an HTTP Request



## 1.4 Writing a Response



## 1.5 Returning Real HTML


## 1.5 Validating the Request and Selectively Responding


## 1.6 A Touch of Refactoring



# 2 Turning Our Single-Threaded Server into a Multithreaded Server



## 2.1 Simulating a Slow Request in the Current Server Implementation


## 2.2 Improving Throughput with a Thread Pool

### 2.2.1 Code Structure If We Could Spawn a Thread for Each Request


### 2.2.2 Creating a Similar Interface for a Finite Number of Threads


### 2.2.3 building the threadpool struct using compiler driven development




### 2.2.4 Validating the Number of Threads in new


### 2.2.5 Creating Space to Store the Threads


### 2.2.6 A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread


### 2.2.7 Sending Requests to Threads via Channels

# 3 Graceful Shutdown and Cleanup



## 3.1 Implementing the Drop Trait on ThreadPool





















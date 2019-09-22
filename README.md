# Xiaoming-s-friends
HangZhou Hackthon 2019-09-22 

为了整洁性，我把 小明的同学们 的所有代码都放在这个文件夹。
For neatness, I put all the code of `Xiaoming's friends` in this folder.

原项目地址：
origin github links:
https://github.com/oumuamua-network

## 目录描述 Directory description

* `./oumuamua-v0.0.1` 是 demo 展示时用到的本地私链。It is the local private chain used in demo presentation. 
* `./oumuamua-refactoring` 是目前正在重构的链，和 `./oumuamua-v0.0.1` 的区别是把不同的功能放在不同的 modules 下面，模块化代码，并且增加了一些功能。It's different from `./oumuamua-v0.0.1` is moduling the different function.
* `./web/` 是前端UI的代码. front-end code.
* `./whitepaper` 白皮书，里面的描述目前大多是愿景。

## 项目介绍 Project introduction
这个项目尝试基于 `Substrate` 来开发一个基于订单模式的借贷系统。里面实现了发币，转币功能，抵押功能，借贷功能等。

更详细的愿景介绍：[whitepaper](./whitepaper/README.md)。 more information for vision, see [whitepaper](./whitepaper/README.md) 

更详细的代码介绍：[oumuamua](./oumuamua-v0.0.1/README.md)。more information for code, see [oumuamua-v0.0.1](./oumuamua-v0.0.1/README.md)

## 未来三个月规划 Plan for the next three months

1. 第一步，我们将重构完代码。然后将其制作成一个类似 Substrate-Kitty https://substrate.dev/substrate-collectables-workshop/#/ 一样的教程
2. 第二步，鉴于 Polkadot 的 btc/eth/eos/tron brigdge 没有 ready，我们将开发基于脚本或智能合约的 btc/eth/eos/tron/ 哈希时间锁定来实现跨链
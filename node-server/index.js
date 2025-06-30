// 引入express框架
const express = require("express");
// 引入cors中间件用于处理跨域请求
const cors = require("cors");

// 创建express应用实例
const app = express();

// 使用cors中间件，允许所有跨域请求
app.use(cors());

// 定义GET路由'/hello'
app.get("/hello", (req, res) => {
  // 返回"hello, node"字符串响应
  res.send("hello, node");
});

// 设置服务器端口号
const PORT = 3000;

// 启动服务器
app.listen(PORT, () => {
  console.log(`服务器运行中，访问 http://localhost:${PORT}/hello 测试API`);
});

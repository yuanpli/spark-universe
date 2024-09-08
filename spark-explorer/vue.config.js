const { defineConfig } = require('@vue/cli-service')
// module.exports = defineConfig({
//   transpileDependencies: true
// })

module.exports = {
  devServer: {
    port: 3000, // 你想要的端口号
    host: 'localhost', // 可以指定主机名
    open: true, // 自动打开浏览器
    https: false, // 是否使用https
    // proxy: 'http://backend-api' // 如果需要设置代理
  },
}
## 启动
### 设置环境变量
```bash
heroku config:set $(cat .env | sed '/^$/d; /#[[:print:]]*$/d')
```
### 提交到**Heroku**
```bash
git push heroku main
```


# 缘由
B站的热门视频是个性化的,  不喜欢的视频拉黑之后不太会出现.  
小号看热门视频会看到大号不想看的视频,  利用这个脚本, 小号也可以屏蔽那些不喜欢的视频.  


# 提前准备

获取大号 还有 小号的 SESSDATA: 登录哔哩哔哩→F12打开控制台→Application→Cookies→SESSDATA  
获取小号的csrf: 登录哔哩哔哩→F12打开控制台→Application→Cookies→bili_jct  


# 过程
- 把step1复制进main.rs,   执行.  收集大号的黑名单. 每50账号/10秒.    结果保存在result.csv.  
- 然后清空main.rs, 把step2的内容放到main.rs.执行.   小号根据大号现有的黑名单, 批量拉黑,  每个账号10秒钟.   






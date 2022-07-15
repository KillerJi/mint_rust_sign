# 离线签名的 Claim

## 获取签名

-   req

    ```http
    GET /sign/{way}/{chain_id}/{buy_way}/{hpn}/{vpn}/{horizontal}/{vertical}/{account}/{nonce}
    ```
    - way: 购买方式
    > 0 商业城 1 世界地图
    - chain_id:链id
    - buy_way:购买方式
    > 0 800商业城 1 1000商业城 2 1300商业城 3 1500商业城 
    > 4 200世界地图 5 400世界地图 6 700世界地图 7 900世界地图
    - hpn:横坐标正负
    > 0 正 1 负 （如：横坐标为-1，该值传1）
    - vpn:纵坐标正负
    > 0 正 1 负 （如：纵坐标为1，该值传0）
    - horizontal:横坐标（输入不用带正负，如-2则传2）
    - vertical：纵坐标（输入不用带正负，如-2则传2）
    - account: 用户地址
    - nonce:合约维护的nonce值
-   res

    ```json
    {
    "buy_way": 4,
    "horizontal": 1,
    "vertical": 2,
    "account": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
    "nonce": 2,
    "v": 27,
    "r": "0xe8a8c3700b038eadb1878e59f3237ffd5cc4d25cc10cd23f84b697ce6d224306",
    "s": "0x39ece4cc0a8ff71459c78cd6531bb6cb411bb2341f91c32bb2bbb86a460f3588"
    }
    ```

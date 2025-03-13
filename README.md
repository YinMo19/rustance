# Rustance

## YinMo19

这是一个使用 rust 编写的记账小程序，可以实现简单的记账功能。使用 sqlite 记录所有数据，因此需要一次迁移 sql 脚本。

This is a simple accounting application written in Rust, designed to manage basic financial records. It uses SQLite to store all data, requiring an initial SQL migration script to set up the database.

## 使用方法

下载 release 中对应平台的二进制文件，并放在一个你觉得合适的地方（只要路径环境变量包含即可）。第一次启动 rustance 进行任何操作会自动创建数据库并执行迁移。之后，就可以使用命令行进行记账了。·

Download the binary file for your platform from the release section and place it in a convenient location (ensure the path is included in your environment variables).On the first run, Rustance will automatically create the database and execute the migration. After that, you can use the command line to manage your accounts.

```sh
> rustance list-all
Creating database /Users/yinmo19/.rustance/database//wallet.db
Create db:/Users/yinmo19/.rustance/database//wallet.db success!

...


> rustance list-all
rustance list-all
                              Accounts Of the Month 2025-03

 amount   in_or_out   append_msg                                 updated_at
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 100      true        test income                                2025-03-13 21:05:52 +08:00
 50.16    true        From 2000_hrs                              2025-03-13 21:06:57 +08:00
 500.16   false       cd: rach karajan                           2025-03-13 21:07:26 +08:00
 100      true        test income                                2025-03-13 21:09:54 +08:00
 100      true        test income test imcome asioaicaoihcsoai   2025-03-13 21:10:16 +08:00
                      choasi asiochaosicosaic ossciashco isahc
                      osaihcosaich osaihcoasihc oasihcoasihcoa
                      sihc oasich oasihc osaihc saoi hc
Statistics━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 150      false       Month Total

> rustance outcome 1500.16 "elec feed"
Inserted outcome record with amount: 1500.16

> rustance list-all
                               Accounts Of the Month 2025-03

 amount    in_or_out   append_msg                                 updated_at
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 100       true        test income                                2025-03-13 21:05:52 +08:00
 50.16     true        From 2000_hrs                              2025-03-13 21:06:57 +08:00
 500.16    false       cd: rach karajan                           2025-03-13 21:07:26 +08:00
 100       true        test income                                2025-03-13 21:09:54 +08:00
 100       true        test income test imcome asioaicaoihcsoai   2025-03-13 21:10:16 +08:00
                       choasi asiochaosicosaic ossciashco isahc
                       osaihcosaich osaihcoasihc oasihcoasihcoa
                       sihc oasich oasihc osaihc saoi hc
 1500.16   false       elec feed                                  2025-03-13 21:36:42 +08:00
Statistics━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 1650.16   false       Month Total
```

文档可以使用 help 查询

For detailed usage, you can use the help command:

```sh
> rustance
Calculate Your Balances.

Usage: rustance <COMMAND>

Commands:
  list-all      List all the Wallet Balances
  income        Add the new income
  outcome       Add the new outcome
  patch-record  patch record
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

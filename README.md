# DBishop
## Drunken Bishop: 醉酒主教

👀 哈希指纹可视化算法, 就像 OpenSSH 那样!

```
> dbishop fc94b0c1e5b0987c5843997697ee9fb7
fingerprint of hex `fc94b0c1e5b0987c5843997697ee9fb7`:
+-----------------+
|       .=o.  .   |
|     . *+*. o    |
|      =.*..o     |
|       o + ..    |
|        S o.     |
|         o  .    |
|          .  . . |
|              o .|
|               E.|
+-----------------+
```

## 使用

```
> dbishop --help
The hash fingerprint visualization algorithm, like OpenSSH

Usage: dbishop [OPTIONS] [DATA]

Arguments:
  [DATA]  Input data, like a hex string

Options:
  -q, --quiet      Don't echo hex input
  -i, --in <FILE>  Use file, one single byte by one; if '-' use stdin
      --story      Read the story of Bishop Peter
  -h, --help       Print help
  -V, --version    Print version
```

## 例子

### 获取一段十六进制字符串的指纹

> e.g. `fc94b0c1e5b0987c5843997697ee9fb7`

```
> dbishop fc94b0c1e5b0987c5843997697ee9fb7
fingerprint of hex `fc94b0c1e5b0987c5843997697ee9fb7`:
+-----------------+
|       .=o.  .   |
|     . *+*. o    |
|      =.*..o     |
|       o + ..    |
|        S o.     |
|         o  .    |
|          .  . . |
|              o .|
|               E.|
+-----------------+
```

### 获取 `base64` 编码的指纹

> e.g. `AMeItYIXNWOp2Qc91TR1iyFWutrVgUfLKCJ3B8/U/QM`

```
> cat data.base64
AMeItYIXNWOp2Qc91TR1iyFWutrVgUfLKCJ3B8/U/QM

> cat data.base64 | base64 -d 2>/dev/null | dbishop -i -
fingerprint of file `-`:
+-----------------+
|  .+B=...o*o=.o. |
| ..o+*+  ..O E.o.|
|. o+..o.o + B B .|
| .o.. .+ o + o o.|
|     .  S . . . .|
|         o .     |
|        . .      |
|                 |
|                 |
+-----------------+
```

### 随机生成一个指纹

```
> dd if=/dev/random bs=1 count=16 2> /dev/null | dbishop -qi -
+-----------------+
|                 |
|         .       |
|        . o      |
|         o .     |
|        S +      |
|       . * +     |
|      o + B      |
|     . o =E=..   |
|        ..**=.   |
+-----------------+
```

## 参考

- [The drunken bishop: An analysis of the OpenSSH fingerprint visualization algorithm
  ](http://dirk-loss.de/sshvis/drunken_bishop.pdf)

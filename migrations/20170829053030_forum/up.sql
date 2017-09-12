CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  email varchar NOT NULL,
  username varchar NOT NULL,
  password varchar NOT NULL,
  created_at timestamp with time zone NOT NULL,
  UNIQUE (email, username)
);

 INSERT INTO users (id, email, username, password, created_at) VALUES
 (1, 'admin@163.com', 'admin', 'admin','2017-07-23 23:41:45.672805609 +08:00'),
 (2, 'zzzz@163.com', 'zzzz', 'zzzz','2017-07-23 23:41:45.672805609 +08:00');
 SELECT setval('users_id_seq', 2, true);


CREATE TABLE  article (
  id SERIAL NOT NULL PRIMARY KEY,
  uid integer NOT NULL,
  category varchar NOT NULL,
  status integer NOT NULL DEFAULT '0',
  comments_count integer NOT NULL DEFAULT '0',
  title varchar NOT NULL,
  content text NOT NULL,
  created_at timestamp with time zone NOT NULL,
  updated_at timestamp with time zone  NOT NULL 
);

 INSERT INTO article (id, uid, category, status, comments_count, title, content, created_at, updated_at) VALUES
 (1, 1, '话题', 0, 0, 'Rust文章', 'generic 可用在 struct, fn,  method,  Bound, trait.\r\n定义封装的模块；\r\n定义新模块对其他模块的依赖；\r\n可对其他模块的引入支持；\r\n那么就需要一套规范准则来定义这些能力', '2017-07-24 23:41:45.672805609 +08:00', '2017-07-23 23:41:45.672805609 +08:00'),
 (2, 2, '文章', 0, 0, 'Rust范型','Rust是一个多范式 (multi-paradigm) 的编译型语言。', '2017-07-23 23:41:45.672805609 +08:00', '2017-07-24 23:41:45.672805609 +08:00'),
 (3, 2, '问答', 0, 0, 'Rust函数','Rust 还支持高阶函数 (high order function)，允许把闭包作为参数来生成新的函数', '2017-07-23 23:41:45.672805609 +08:00', '2017-07-24 23:41:45.672805609 +08:00'),
 (4, 1, '分享', 0, 0, 'Rust文章', 'generic 可用在 struct, fn,  method,  Bound, trait.\r\n定义封装的模块；\r\n定义新模块对其他模块的依赖；\r\n可对其他模块的引入支持；\r\n那么就需要一套规范准则来定义这些能力', '2017-07-24 23:41:45.672805609 +08:00', '2017-07-23 23:41:45.672805609 +08:00'),
 (5, 2, '分享', 0, 0, 'Rust范型','Rust是一个多范式 (multi-paradigm) 的编译型语言。', '2017-07-23 23:41:45.672805609 +08:00', '2017-07-24 23:41:45.672805609 +08:00'),
 (6, 2, '话题', 0, 0, 'Rust函数','Rust 还支持高阶函数 (high order function)，允许把闭包作为参数来生成新的函数', '2017-07-23 23:41:45.672805609 +08:00', '2017-07-24 23:41:45.672805609 +08:00');
 SELECT setval('article_id_seq', 6, true);


CREATE TABLE  comment (
  id SERIAL NOT NULL PRIMARY KEY,
  aid integer NOT NULL,
  uid integer NOT NULL,
  content text NOT NULL,
  created_at timestamp with time zone NOT NULL
);

 INSERT INTO comment (id, aid, uid, content, created_at) VALUES
 (1, 1, 1, '好，非常不错', '2017-07-23 23:41:45.672805609 +08:00'),
 (2, 1, 1, '数据库连接池正是针对这个问题提出来的，它会负责分配、管理和释放数据库连接，允许应用程序重复使用一个现有的数据库连接，而不是重新建立一个连接，释放空闲时间超过最大允许空闲时间的数据库连接以避免因为连接未释放而引起的数据库连接遗漏。', '2017-07-23 23:41:45.672805609 +08:00'),
 (3, 1, 2, 'Rust通过`impl`关键字在`struct`、`enum`或者`trait`对象上实现方法调用语法', '2017-07-23 23:41:45.672805609 +08:00'),
 (4, 1, 2, '泛型 (generics) 在类型理论中称作参数多态 (parametric polymorphism)，意为对于给定参数可以有多种形式的函数或类型。！', '2017-07-23 23:41:45.672805609 +08:00'),
 (5, 1, 2, '特性也可以接受泛型参数。但是，往往更好的处理方式是使用关联类型！', '2017-07-23 23:41:45.672805609 +08:00'),
 (6, 1, 2, 'Rust提供了两个特性来处理并发 (concurrency)：`Send`和`Sync`', '2017-07-23 23:41:45.672805609 +08:00'),
 (7, 2, 2, 'Rust尝试解决可变状态的共享问题，通过所有权系统来帮助排除数据竞争', '2017-07-23 23:41:45.672805609 +08:00'),
 (8, 2, 2, '同一时间只允许一个线程能修改它的值。`mpsc::channel()`方法创建了一个通道 (channel)', '2017-07-23 23:41:45.672805609 +08:00');
 SELECT setval('comment_id_seq', 8, true);


CREATE TABLE message (
  id SERIAL NOT NULL PRIMARY KEY,
  aid integer NOT NULL,
  cid integer NOT NULL,
  from_uid integer NOT NULL,
  to_uid integer NOT NULL,
  content text NOT NULL,
  mode integer NOT NULL DEFAULT '0',
  status integer NOT NULL DEFAULT '0',
  created_at timestamp with time zone NOT NULL
);



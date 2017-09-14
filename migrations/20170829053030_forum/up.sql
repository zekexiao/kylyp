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
 (1, 1, 'Topic', 0, 2, 'Rust Article', 'Rust 2017 Survey Results', '2017-07-24 23:41:45.672805609 +08:00', '2017-07-23 23:41:45.672805609 +08:00'),
 (2, 2, 'Article', 0, 3, 'The Rust Libz Blitz','This post covers the library team’s major initiative: raising a solid core of the Rust crate ecosystem to a consistent level of completeness and quality. ', '2017-07-23 23:41:45.672805609 +08:00', '2017-07-24 23:41:45.672805609 +08:00'),
 (3, 2, 'QAF', 0, 1, 'Rust 2017 roadmap','This year, the overarching theme is productivity, especially for early-stage Rust users. ', '2017-07-23 23:41:45.672805609 +08:00', '2017-07-24 23:41:45.672805609 +08:00'),
 (4, 1, 'Share', 0, 1, 'Incremental Compilation', 'One of the projects that is building on these foundations, and that should help improve compile times a lot for typical workflows, is incremental compilation. ', '2017-07-24 23:41:45.672805609 +08:00', '2017-07-23 23:41:45.672805609 +08:00'),
 (5, 2, 'Job', 0, 1, 'Rust jobs','Today we are announcing an alpha version of incremental compilation', '2017-07-23 23:41:45.672805609 +08:00', '2017-07-24 23:41:45.672805609 +08:00'),
 (6, 2, 'Blog', 0, 0, 'Introducing MIR','MIR is the key to ticking off a number of our highest priorities for Rust', '2017-07-23 23:41:45.672805609 +08:00', '2017-07-24 23:41:45.672805609 +08:00');
 SELECT setval('article_id_seq', 6, true);


CREATE TABLE  comment (
  id SERIAL NOT NULL PRIMARY KEY,
  aid integer NOT NULL,
  uid integer NOT NULL,
  content text NOT NULL,
  created_at timestamp with time zone NOT NULL
);

 INSERT INTO comment (id, aid, uid, content, created_at) VALUES
 (1, 1, 1, 'Faster execution time', '2017-07-23 23:41:45.672805609 +08:00'),
 (2, 1, 1, 'Faster compilation time', '2017-07-23 23:41:45.672805609 +08:00'),
 (3, 3, 2, 'More precise type checking.', '2017-07-23 23:41:45.672805609 +08:00'),
 (4, 2, 2, 'Eliminating redundancy！', '2017-07-23 23:41:45.672805609 +08:00'),
 (5, 4, 2, 'Raising ambitions.！', '2017-07-23 23:41:45.672805609 +08:00'),
 (6, 5, 2, 'MIR construction is type-driven', '2017-07-23 23:41:45.672805609 +08:00'),
 (7, 2, 2, 'Some MIR primitives are more powerful than the structured construct they replace', '2017-07-23 23:41:45.672805609 +08:00'),
 (8, 2, 2, 'MIR makes all types explicit', '2017-07-23 23:41:45.672805609 +08:00');
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

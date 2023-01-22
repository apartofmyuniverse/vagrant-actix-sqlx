
CREATE SEQUENCE IF NOT EXISTS users_id_seq;

CREATE TABLE IF NOT EXISTS public.users (
      id int8 NOT NULL DEFAULT nextval('users_id_seq'::regclass),
      first_name varchar NOT NULL,
      last_name varchar NOT NULL,
      PRIMARY KEY (id)
      );

CREATE SEQUENCE IF NOT EXISTS articles_id_seq;

CREATE TABLE IF NOT EXISTS public.articles (
      id int8 NOT NULL DEFAULT nextval('articles_id_seq'::regclass),
      title varchar NOT NULL,
      content varchar NOT NULL,
      created_by int8 NOT NULL,
      PRIMARY KEY (id)
      );
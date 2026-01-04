# Assignment : Movie Database SQL Queries

This document contains 50 SQL queries for a movie database management system. Each query includes the question, SQL solution, and Output format.

---

## Query 1

**Question:** Write a SQL query to find the name and year of the movies. Return movie title, movie release year.

**SQL Query:**

```sql
SELECT
	mov_title AS movie_title , mov_year AS movie_release_year
FROM
	movie;
```

**Output:**
| movie_title | movie_release_year |
|-----------------------------|------------------|
| Vertigo | 1958 |
| The Innocents | 1961 |
| Lawrence of Arabia | 1962 |
| The Deer Hunter | 1978 |
| Amadeus | 1984 |
| Blade Runner | 1982 |
| Eyes Wide Shut | 1999 |
| The Usual Suspects | 1995 |
| Chinatown | 1974 |
| Boogie Nights | 1997 |
| Annie Hall | 1977 |
| Princess Mononoke | 1997 |
| The Shawshank Redemption | 1994 |
| American Beauty | 1999 |
| Titanic | 1997 |
| Good Will Hunting | 1997 |
| Deliverance | 1972 |
| Trainspotting | 1996 |
| The Prestige | 2006 |
| Donnie Darko | 2001 |
| Slumdog Millionaire | 2008 |
| Aliens | 1986 |
| Beyond the Sea | 2004 |
| Avatar | 2009 |
| Seven Samurai | 1954 |
| Spirited Away | 2001 |
| Back to the Future | 1985 |
| Braveheart | 1995 |

---

## Query 2

**Question:** Write a SQL query to find when the movie 'American Beauty' released. Return movie release year.

**SQL Query:**

```sql
SELECT
	mov_year AS movie_release_year
FROM
	movie
WHERE
	mov_title = "American Beauty";
```

**Output:**
| movie_release_year |
|-------------------|
| 1999 |

---

## Query 3

**Question:** Write a SQL query to find the movie that was released in 1999. Return movie title.

**SQL Query:**

```sql
SELECT
	mov_title AS movie_title
FROM
	movie
WHERE
	mov_year=1999;
```

**Output:**
| movie_title |
|-------------|
Eyes Wide Shut
American Beauty

---

## Query 4

**Question:** Write a SQL query to find those movies, which were released before 1998. Return movie title.

**SQL Query:**

```sql
SELECT
	mov_title AS movie_title
FROM
	movie
WHERE
	mov_year < 1998;
```

**Output:**
| movie_title |
|-------------|
Vertigo
The Innocents
Lawrence of Arabia
The Deer Hunter
Amadeus
Blade Runner
The Usual Suspects
Chinatown
Boogie Nights
Annie Hall
Princess Mononoke
The Shawshank Redemption
Titanic
Good Will Hunting
Deliverance
Trainspotting
Aliens
Seven Samurai
Back to the Future
Braveheart

---

## Query 5

**Question:** Write a SQL query to find the name of all reviewers and movies together in a single list.

**SQL Query:**

```sql
SELECT
	rev_name AS name
FROM
	movie_reviewer
UNION
SELECT
	mov_title AS name
FROM
	movie;
```

**Output:**
| name |
|------|
Righty Sock
Jack Malvern
Flagrant Baronessa
Alec Shaw
NULL
Victor Woeltjen
Simon Wright
Neal Wruck
Paul Monks
Mike Salvati
Wesley S. Walker
Sasha Goldshtein
Josh Cates
Krug Stillo
Scott LeBrun
Hannah Steele
Vincent Cadena
Brandt Sponseller
Richard Adams
Vertigo
The Innocents
Lawrence of Arabia
The Deer Hunter
Amadeus
Blade Runner
Eyes Wide Shut
The Usual Suspects
Chinatown
Boogie Nights
Annie Hall
Princess Mononoke
The Shawshank Redemption
American Beauty
Titanic
Good Will Hunting
Deliverance
Trainspotting
The Prestige
Donnie Darko
Slumdog Millionaire
Aliens
Beyond the Sea
Avatar
Seven Samurai
Spirited Away
Back to the Future
Braveheart

---

## Query 6

**Question:** Write a SQL query to find all reviewers who have rated seven or more stars to their rating. Return reviewer name.

**SQL Query:**

```sql
SELECT
	mrw.rev_name AS reviewer_name
FROM
	movie_reviewer mrw JOIN movie_rating mrt
	ON
	mrw.rev_id = mrt.rev_id
WHERE
	mrt.rev_stars >= 7;
```

**Output:**
| reviewer_name |
|---------------------|
| Righty Sock |
| Jack Malvern |
| Flagrant Baronessa |
|NULL|
| Simon Wright |
| Mike Salvati |
| Sasha Goldshtein |
| Righty Sock |
| Hannah Steele |
| Vincent Cadena |
| Brandt Sponseller |
| Victor Woeltjen |
| Krug Stillo |

---

## Query 7

**Question:** Write a SQL query to find the movies without any rating. Return movie title.

**SQL Query:**

```sql
SELECT
	m.mov_title AS movie_title
FROM
	movie m JOIN movie_rating mrt
    ON
    m.mov_id = mrt.mov_id
WHERE
	mrt.num_o_ratings is null;
```

**Output:**
| movie_title |
|-------------|
Princess Mononoke
Avatar

---

## Query 8

**Question:** Write a SQL query to find the movies with ID 905 or 907 or 917. Return movie title.

**SQL Query:**

```sql
SELECT
	mov_title AS movie_name
FROM
	movie
WHERE
	mov_id IN (905, 907, 917);
```

**Output:**
| movie_name |
|------------|

---

## Query 9

**Question:** Write a SQL query to find the movie titles that contain the word 'Boogie Nights'. Sort the result-set in ascending order by movie year. Return movie ID, movie title and movie release year.

**SQL Query:**

```sql
SELECT
	mov_id AS movie_ID,
    mov_title AS movie_title,
    mov_year AS movie_release_year
FROM
	movie
WHERE
	mov_title LIKE "%Boogie Nights%"
ORDER BY
	mov_year ASC;
```

**Output:**
| movie_ID | movie_title | movie_release_year |
|----------|-------------|-------------------|
| 10 | Boogie Nights | 1997 |

---

## Query 10

**Question:** Write a SQL query to find those actors with the first name 'Woody' and the last name 'Allen'. Return actor ID.

**SQL Query:**

```sql
SELECT
	act_id AS actor_ID
FROM
	actor
WHERE
	act_fname = 'Woody'
    AND
    acrt_lname = 'Allen';
```

**Output:**
| actor_ID |
|----------|
| 11 |

---

# Sub-Queries:

## Query 11

**Question:** Write a SQL query to find the actors who played a role in the movie 'Annie Hall'. Return all the fields of actor table.

**SQL Query:**

```sql
SELECT * FROM actor
WHERE
	act_id IN
	(SELECT mc.act_id FROM
		movie_cast mc
	WHERE
		mc.mov_id IN
		(SELECT mov_id FROM
			movie
		WHERE
			mov_title = 'Annie Hall'
		)
	);
```

**Output:**
| act_id | act_fname | acrt_lname | act_gender |
|--------|-----------|------------|------------|
| 11 | Woody | Allen | M |

---

## Query 12

**Question:** Write a SQL query to find the director of a film that cast a role in 'Eyes Wide Shut'. Return director first name, last name.

**SQL Query:**

```sql
SELECT
	dir_fname AS director_first_name,
    dir_lname AS director_last_name
FROM
	director
WHERE dir_id
	IN
	(SELECT dir_id FROM
		movie_direction
	WHERE
		dir_id IN (SELECT mov_id FROM
		movie
	WHERE
		mov_title = 'Eyes Wide Shut')
	);
```

**Output:**
| director_first_name | director_last_name |
|---------------------|-------------------|
| Stanley | Kubrick |

---

## Query 13

**Question:** Write a SQL query to find those movies that have been released in countries other than the United Kingdom. Return movie title, movie year, movie time, and date of release, releasing country.

**SQL Query:**

```sql
SELECT
	mov_title, mov_year, mov_time, mov_dt_rel, mov_rel_country
FROM
	movie
WHERE mov_id
	IN
    ( SELECT mov_id
	FROM movie
	WHERE mov_rel_country != 'UK'
);
```

**Output:**
| mov_title | mov_year | mov_time | mov_dt_rel | mov_rel_country |
|-----------------|----------|----------|------------|----------------|
| The Innocents | 1961 | 100 | 1962-02-19 | SW |
| Annie Hall | 1977 | 93 | 1977-04-20 | USA |
| Seven Samurai | 1954 | 207 | 1954-04-26 | JP |

---

## Query 14

**Question:** Write a SQL query to find for movies whose reviewer is unknown. Return movie title, year, release date, director first name, last name, actor first name, last name.

**SQL Query:**

```sql
SELECT
    mov_title,
    mov_year,
    mov_dt_rel,
    (SELECT dir_fname
     FROM director
     WHERE dir_id = (SELECT dir_id
                     FROM movie_direction
                     WHERE mov_id = m.mov_id)) AS director_fname,
    (SELECT dir_lname
     FROM director
     WHERE dir_id = (SELECT dir_id
                     FROM movie_direction
                     WHERE mov_id = m.mov_id)) AS director_lname,
    (SELECT act_fname
     FROM actor
     WHERE act_id = (SELECT act_id
                     FROM movie_cast
                     WHERE mov_id = m.mov_id)) AS actor_fname,
    (SELECT acrt_lname
     FROM actor
     WHERE act_id = (SELECT act_id
                     FROM movie_cast
                     WHERE mov_id = m.mov_id)) AS actor_lname
FROM movie m
WHERE
	mov_id
	IN
(SELECT mov_id
 FROM movie_rating
 WHERE rev_id
	IN
    (SELECT rev_id
	 FROM movie_reviewer
	 WHERE rev_name = '')
);
```

**Output:**
| mov_title | mov_year | mov_dt_rel | director_fname | director_lname | actor_fname | actor_lname |
|-------------------|----------|------------|----------------|----------------|-------------|-------------|
| Blade Runner | 1982 | 1982-09-09 | Ridley | Scott | Harrison | Ford |
| Princess Mononoke | 1997 | 2001-10-19 | Hayao | Miyazaki | Claire | Danes |

---

## Query 15

**Question:** Write a SQL query to find those movies directed by the director whose first name is Woddy and last name is Allen. Return movie title.

**SQL Query:**

```sql
SELECT mov_title AS movie_title
FROM movie
WHERE mov_id IN (SELECT mov_id
				FROM movie_direction
				WHERE dir_id IN
					(SELECT dir_id
					FROM director
					WHERE
					dir_fname = 'Woody' AND dir_lname='Allen')
				);
```

**Output:**
| movie_title |
|-------------|
| Annie Hall |

---

## Query 16

**Question:** Write a SQL query to determine those years in which there was at least one movie that received a rating of at least three stars. Sort the result-set in ascending order by movie year. Return movie year.

**SQL Query:**

```sql
SELECT mov_year AS movie_year
FROM movie
WHERE mov_id IN
			(SELECT mov_id
			FROM movie_rating
			WHERE rev_stars >= 3)
            ORDER BY mov_year ASC;
```

**Output:**
| movie_year |
|------------|
1954
1958
1961
1962
1977
1982
1986
1995
1997
1997
1997
1997
1999
2001
2004
2008
2009

---

## Query 17

**Question:** Write a SQL query to search for movies that do not have any ratings. Return movie title.

**SQL Query:**

```sql
SELECT mov_title AS movie_title
FROM movie
WHERE mov_id IN
			(SELECT mov_id
			FROM movie_rating
			WHERE rev_stars IS NULL);
```

**Output:**
| movie_title |
|-------------|
Chinatown
Trainspotting

---

## Query 18

**Question:** Write a SQL query to find those reviewers who have not given a rating to certain films. Return reviewer name.

**SQL Query:**

```sql
SELECT rev_name AS reviewer_name
FROM movie_reviewer
WHERE rev_id NOT IN
			(SELECT rev_id
			FROM movie_rating );
```

**Output:**
| reviewer_name |
|---------------|
reviewer_name
Alec Shaw
Wesley S. Walker

---

## Query 19

**Question:** Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Sort the result-set in ascending order by reviewer name, movie title, review Stars. Return reviewer name, movie title, review Stars.

**SQL Query:**

```sql
SELECT
		(SELECT r.rev_name
        FROM movie_reviewer r
        WHERE r.rev_id = mr.rev_id) AS reviewer_name,
		(SELECT m.mov_title
        FROM movie m
        WHERE m.mov_id = mr.mov_id) AS movie_title,
        mr.rev_stars AS review_stars
FROM movie_rating mr
WHERE mr.rev_stars IS NOT NULL
ORDER BY
reviewer_name ASC,
movie_title ASC,
review_stars ASC;
```

**Output:**
| reviewer_name | movie_title | review_stars |
|-----------------------|------------------------|--------------|
| NULL | Blade Runner | 8.2 |
| NULL | Princess Mononoke | 8.4 |
| Brandt Sponseller | Aliens | 8.4 |
| Flagrant Baronessa | Lawrence of Arabia | 8.3 |
| Hannah Steele | Donnie Darko | 8.1 |
| Jack Malvern | The Innocents | 7.9 |
| Josh Cates | Good Will Hunting | 4.0 |
| Krug Stillo | Seven Samurai | 7.7 |
| Mike Salvati | Annie Hall | 8.1 |
| Paul Monks | Boogie Nights | 3.0 |
| Richard Adams | Beyond the Sea | 6.7 |
| Righty Sock | Titanic | 7.7 |
| Righty Sock | Vertigo | 8.4 |
| Sasha Goldshtein | American Beauty | 7.0 |
| Simon Wright | The Usual Suspects | 8.6 |
| Victor Woeltjen | Avatar | 7.3 |
| Vincent Cadena | Slumdog Millionaire | 8.0 |

---

## Query 20

**Question:** Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Group the result set on reviewer’s name, movie title. Return reviewer’s name, movie title.

**SQL Query:**

```sql
SELECT
		(SELECT r.rev_name
        FROM movie_reviewer r
        WHERE r.rev_id = mr.rev_id) AS reviewer_name,
		(SELECT m.mov_title
        FROM movie m
        WHERE m.mov_id = mr.mov_id) AS movie_title
FROM movie_rating mr
WHERE mr.rev_stars IS NOT NULL
GROUP BY movie_title , reviewer_name;
```

**Output:**
| reviewer_name | movie_title |
|-----------------------|------------------------|
| Righty Sock | Vertigo |
| Jack Malvern | The Innocents |
| Flagrant Baronessa | Lawrence of Arabia |
| | Blade Runner |
| Simon Wright | The Usual Suspects |
| Paul Monks | Boogie Nights |
| Mike Salvati | Annie Hall |
| | Princess Mononoke |
| Sasha Goldshtein | American Beauty |
| Righty Sock | Titanic |
| Josh Cates | Good Will Hunting |
| Hannah Steele | Donnie Darko |
| Vincent Cadena | Slumdog Millionaire |
| Brandt Sponseller | Aliens |
| Richard Adams | Beyond the Sea |
| Victor Woeltjen | Avatar |
| Krug Stillo | Seven Samurai |

---

## Query 21

**Question:** Write a SQL query to find those movies, which have received highest number of stars. Group the result set on movie title and sorts the result-set in ascending order by movie title. Return movie title and maximum number of review stars.

**SQL Query:**

```sql
SELECT m.mov_title, r.rev_stars
FROM movie m, movie_rating r
WHERE m.mov_id = r.mov_id
  AND r.rev_stars >= ALL (
        SELECT rev_stars
        FROM movie_rating
        WHERE rev_stars IS NOT NULL
  )
group by m.mov_title , r.rev_stars
order by m.mov_title;
```

**Output:**
| mov_title | rev_stars |
|-----------|-----------|
| The Usual Suspects| 8.6|

---

## Query 22

**Question:** Write a SQL query to find all reviewers who rated the movie 'American Beauty'. Return reviewer name.

**SQL Query:**

```sql
SELECT mrv.rev_name AS reviewer_name
FROM movie_reviewer mrv
WHERE mrv.rev_id IN (SELECT mr.rev_id
					FROM movie_rating mr
					WHERE mr.mov_id IN (SELECT mov_id
										FROM movie
										WHERE mov_title = 'American Beauty')
					);
```

**Output:**
| reviewer_name |
|---------------|
Sasha Goldshtein

---

## Query 23

**Question:** Write a SQL query to find the movies that have not been reviewed by any reviewer body other than 'Paul Monks'. Return movie title.

**SQL Query:**

```sql
SELECT m.mov_title AS movie_title
FROM movie m
WHERE m.mov_id IN
				(SELECT mr.mov_id
				FROM movie_rating mr
				WHERE mr.rev_id IN (SELECT mrv.rev_id
									FROM movie_reviewer AS mrv
									WHERE mrv.rev_name = 'Paul Monks')
									);
```

**Output:**
| movie_title |
|-------------|
Boogie Nights

---

## Query 24

**Question:** Write a SQL query to find the movies with the lowest ratings. Return reviewer name, movie title, and number of stars for those movies.

**SQL Query:**

```sql
SELECT
    (SELECT mr.rev_name
     FROM movie_reviewer mr
     WHERE mr.rev_id = r.rev_id) AS reviewer_name,
    (SELECT m.mov_title
     FROM movie m
     WHERE m.mov_id = r.mov_id) AS movie_title,
    r.rev_stars AS review_stars
FROM movie_rating r
WHERE r.rev_stars <= ALL (SELECT rev_stars
						FROM movie_rating
						WHERE rev_stars IS NOT NULL
					);
```

**Output:**
| reviewer_name | movie_title | review_stars |
|---------------|-------------|--------------|
| Paul Monks |Boogie Nights| 3.0|

---

## Query 25

**Question:** Write a SQL query to find the movies directed by 'James Cameron'. Return movie title.

**SQL Query:**

```sql
SELECT mov_title AS movie_title
FROM movie WHERE mov_id IN
					(SELECT mov_id
					FROM movie_direction
					WHERE dir_id IN (SELECT dir_id
									FROM director
									WHERE dir_fname = 'James' AND dir_lname = 'Cameron')
					);
```

**Output:**
| movie_title |
|-------------|
Titanic
Aliens

---

## Query 26

**Question:** Write a query in SQL to find the movies in which one or more actors appeared in more than one film.

**SQL Query:**

```sql

SELECT DISTINCT m.mov_title
FROM movie m
WHERE m.mov_id IN
				(SELECT mc.mov_id
				FROM movie_cast mc
				WHERE mc.act_id IN
								(SELECT act_id
								FROM movie_cast
								GROUP BY act_id
								HAVING COUNT(DISTINCT mov_id) > 1
    )
);
```

**Output:**
| mov_title |
|-----------|
American Beauty
Beyond the Sea

---

## Query 27

**Question:** Write a SQL query to find all reviewers whose ratings contain a NULL value. Return reviewer name.

**SQL Query:**

```sql
SELECT mrv.rev_name AS reviewer_name
FROM
	movie_reviewer mrv
JOIN
	movie_rating mr
ON mrv.rev_id = mr.rev_id
WHERE mr.rev_stars IS NULL;
```

**Output:**
| reviewer_name |
|---------------|
Neal Wruck
Scott LeBrun

---

## Query 28

**Question:** Write a SQL query to find out who was cast in the movie 'Annie Hall'. Return actor first name, last name and role.

**SQL Query:**

```sql
SELECT a.act_fname AS first_name , a.acrt_lname AS last_name , mc.role
FROM actor a
JOIN
	movie_cast mc
ON a.act_id = mc.act_id
JOIN
	movie m
ON
	m.mov_id = mc.mov_id
WHERE m.mov_title = 'Annie Hall';
```

**Output:**
| first_name | last_name | role |
|------------|-----------|------|
| Woody | Allen | Alvy Singer |

---

## Query 29

**Question:** Write a SQL query to find the director who directed a movie that featured a role in 'Eyes Wide Shut'. Return director first name, last name and movie title.

**SQL Query:**

```sql

SELECT d.dir_fname , d.dir_lname , m.mov_title AS movie_title
FROM director d
JOIN
	movie_direction md
ON
	d.dir_id = md.dir_id
JOIN
	movie m
ON
	m.mov_id = md.mov_id
WHERE
	m.mov_title = 'Eyes Wide Shut';

```

**Output:**
| dir_fname | dir_lname | movie_title |
|-----------|-----------|-------------|
| Stanley | Kubrick | Eyes Wide Shut |

---

## Query 30

**Question:** Write a SQL query to find the director of a movie that cast a role as Sean Maguire. Return director first name, last name and movie title.

**SQL Query:**

```sql
SELECT d.dir_fname,d.dir_lname,m.mov_title AS movie_title
FROM
	director d
JOIN
	movie_direction md
ON d.dir_id = md.dir_id
JOIN
	movie_cast mc
ON
	mc.mov_id = md.mov_id
JOIN
	movie m
ON
	md.mov_id = m.mov_id
WHERE
	mc.role = 'Sean Maguire';
```

**Output:**
| dir_fname | dir_lname | movie_title |
|-----------|-----------|-------------|
| Gus | Van Sant | Good Will Hunting |

---

## Query 31

**Question:** Write a SQL query to find out which actors have not appeared in any movies between 1990 and 2000 (Begin and end values are included.). Return actor first name, last name, movie title and release year.

**SQL Query:**

```sql
SELECT
    a.act_fname,
    a.acrt_lname,
    m.mov_title,
    m.mov_year
FROM actor a
JOIN movie_cast mc ON a.act_id = mc.act_id
JOIN movie m ON mc.mov_id = m.mov_id
WHERE a.act_id NOT IN
						(SELECT mc2.act_id
						FROM movie_cast mc2
						JOIN movie m2 ON mc2.mov_id = m2.mov_id
						WHERE m2.mov_year BETWEEN 1990 AND 2000
					);
```

**Output:**
| act_fname | acrt_lname | mov_title | mov_year |
|----------------|-----------------|----------------------|----------|
| James | Stewart | Vertigo | 1958 |
| Deborah | Kerr | The Innocents | 1961 |
| Peter | OToole | Lawrence of Arabia | 1962 |
| Robert | De Niro | The Deer Hunter | 1978 |
| F. Murray | Abraham | Amadeus | 1984 |
| Harrison | Ford | Blade Runner | 1982 |
| Jack | Nicholson | Chinatown | 1974 |
| Christian | Bale | Chinatown | 1974 |
| Woody | Allen | Annie Hall | 1977 |
| Jon | Voight | Deliverance | 1972 |
| Maggie | Gyllenhaal | Donnie Darko | 2001 |
| Dev | Patel | Slumdog Millionaire | 2008 |
| Sigourney | Weaver | Aliens | 1986 |

---

## Query 32

**Question:** Write a SQL query to find the directors who have directed films in a variety of genres. Group the result set on director first name, last name and generic title. Sort the result-set in ascending order by director first name and last name. Return director first name, last name and number of genres movies.

**SQL Query:**

```sql
SELECT
    d.dir_fname,
    d.dir_lname,
    COUNT(DISTINCT g.gen_id) AS number_of_genres
FROM director d
JOIN movie_direction md
    ON d.dir_id = md.dir_id
JOIN movie_genres mg
    ON md.mov_id = mg.mov_id
JOIN genres g
    ON mg.gen_id = g.gen_id
GROUP BY
    d.dir_fname,
    d.dir_lname
HAVING COUNT(DISTINCT g.gen_title) > 1
ORDER BY
    d.dir_fname ASC,
    d.dir_lname ASC;
```

**Output:**
| dir_fname | dir_lname | number_of_genres |
|-----------|-----------|------------------|

---

## Query 33

**Question:** Write a SQL query to find the movies with year and genres. Return movie title, movie year and generic title.

**SQL Query:**

```sql
SELECT m.mov_title AS movie_title , m.mov_year AS movie_year , g.gen_title
FROM movie m
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON g.gen_id = mg.gen_id;
```

**Output:**
| movie_title | movie_year | gen_title |
|---------------------------|------------|------------|
| Aliens | 1986 | Action |
| Lawrence of Arabia | 1962 | Adventure |
| Deliverance | 1972 | Adventure |
| Princess Mononoke | 1997 | Animation |
| Annie Hall | 1977 | Comedy |
| The Usual Suspects | 1995 | Crime |
| The Shawshank Redemption | 1994 | Crime |
| Trainspotting | 1996 | Drama |
| Slumdog Millionaire | 2008 | Drama |
| Spirited Away | 2001 | Drama |
| Braveheart | 1995 | Drama |
| The Innocents | 1961 | Horror |
| Beyond the Sea | 2004 | Music |
| Vertigo | 1958 | Mystery |
| Eyes Wide Shut | 1999 | Mystery |
| Back to the Future | 1985 | Mystery |
| American Beauty | 1999 | Romance |
| Blade Runner | 1982 | Thriller |
| The Deer Hunter | 1978 | War |

---

## Query 34

**Question:** Write a SQL query to find all the movies with year, genres, and name of the director.

**SQL Query:**

```sql
SELECT
    m.mov_title,
    m.mov_year,
    g.gen_title AS genre,
    d.dir_fname,
    d.dir_lname
FROM movie m
JOIN movie_genres mg
    ON m.mov_id = mg.mov_id
JOIN genres g
    ON mg.gen_id = g.gen_id
JOIN movie_direction md
    ON m.mov_id = md.mov_id
JOIN director d
    ON md.dir_id = d.dir_id;
```

**Output:**
| mov_title | mov_year | genre | dir_fname | dir_lname |
|--------------------------|----------|-----------|------------|-----------|
| Aliens | 1986 | Action | James | Cameron |
| Lawrence of Arabia | 1962 | Adventure | David | Lean |
| Deliverance | 1972 | Adventure | John | Boorman |
| Princess Mononoke | 1997 | Animation | Hayao | Miyazaki |
| Annie Hall | 1977 | Comedy | Woody | Allen |
| The Usual Suspects | 1995 | Crime | Bryan | Singer |
| The Shawshank Redemption | 1994 | Crime | Frank | Darabont |
| Trainspotting | 1996 | Drama | Danny | Boyle |
| Slumdog Millionaire | 2008 | Drama | Danny | Boyle |
| The Innocents | 1961 | Horror | Jack | Clayton |
| Beyond the Sea | 2004 | Music | Kevin | Spacey |
| Vertigo | 1958 | Mystery | Alfred | Hitchcock |
| Eyes Wide Shut | 1999 | Mystery | Stanley | Kubrick |
| American Beauty | 1999 | Romance | Sam | Mendes |
| Blade Runner | 1982 | Thriller | Ridley | Scott |
| The Deer Hunter | 1978 | War | Michael | Cimino |

---

## Query 35

**Question:** Write a SQL query to find the movies released before 1st January 1989. Sort the result-set in descending order by date of release. Return movie title, release year, date of release, duration, and first and last name of the director.

**SQL Query:**

```sql
SELECT
    m.mov_title,
    m.mov_year,
    m.mov_dt_rel,
    m.mov_time,
    d.dir_fname,
    d.dir_lname
FROM movie m
JOIN movie_direction md
    ON m.mov_id = md.mov_id
JOIN director d
    ON md.dir_id = d.dir_id
WHERE m.mov_dt_rel < '1989-01-01'
ORDER BY m.mov_dt_rel DESC;
```

**Output:**
| mov_title | mov_year | mov_dt_rel | mov_time | dir_fname | dir_lname |
|--------------------|----------|------------|----------|-----------|-----------|
| Aliens | 1986 | 1986-08-29 | 137 | James | Cameron |
| Amadeus | 1984 | 1985-01-07 | 160 | Milos | Forman |
| Deliverance | 1972 | 1982-10-05 | 109 | John | Boorman |
| Blade Runner | 1982 | 1982-09-09 | 117 | Ridley | Scott |
| The Deer Hunter | 1978 | 1979-03-08 | 183 | Michael | Cimino |
| Annie Hall | 1977 | 1977-04-20 | 93 | Woody | Allen |
| Chinatown | 1974 | 1974-08-09 | 130 | Roman | Polanski |
| Lawrence of Arabia | 1962 | 1962-12-11 | 216 | David | Lean |
| The Innocents | 1961 | 1962-02-19 | 100 | Jack | Clayton |
| Vertigo | 1958 | 1958-08-24 | 128 | Alfred | Hitchcock |

---

## Query 36

**Question:** Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.

**SQL Query:**

```sql
SELECT g.gen_title , AVG(m.mov_time) AS avg_movie_time , COUNT(m.mov_id) AS no_of_movies_in_genre
FROM genres g
JOIN movie_genres mg ON g.gen_id = mg.gen_id
JOIN movie m ON m.mov_id = mg.mov_id
GROUP BY  g.gen_title;
```

**Output:**
| gen_title | avg_movie_time | no_of_movies_in_genre |
|------------|----------------|----------------------|
| Action | 137.0000 | 1 |
| Adventure | 162.5000 | 2 |
| Animation | 134.0000 | 1 |
| Comedy | 93.0000 | 1 |
| Crime | 124.0000 | 2 |
| Drama | 129.2500 | 4 |
| Horror | 100.0000 | 1 |
| Music | 118.0000 | 1 |
| Mystery | 134.3333 | 3 |
| Romance | 122.0000 | 1 |
| Thriller | 117.0000 | 1 |
| War | 183.0000 | 1 |

---

## Query 37

**Question:** Write a SQL query to find movies with the shortest duration. Return movie title, movie year, director first name, last name, actor first name, last name and role.

**SQL Query:**

```sql
SELECT m.mov_title , m.mov_year , d.dir_fname,d.dir_lname , a.act_fname, a.acrt_lname , mc.role
FROM
movie m JOIN movie_cast mc ON mc.mov_id = m.mov_id
		JOIN actor a ON a.act_id = mc.act_id
        JOIN movie_direction md ON md.mov_id = m.mov_id
        JOIN director d ON d.dir_id = md.dir_id
WHERE
	m.mov_time <= ALL(SELECT mov_time FROM movie);
```

**Output:**
| mov_title | mov_year | dir_fname | dir_lname | act_fname | acrt_lname | role |
|-----------|----------|-----------|-----------|-----------|------------|------|
| Annie Hall| 1977| Woody| Allen| Woody| Allen| Alvy Singer|

---

## Query 38

**Question:** Write a SQL query to find the years in which a movie received a rating of 3 or 4. Sort the result in increasing order on movie year.

**SQL Query:**

```sql
SELECT m.mov_year AS movie_year
FROM
movie m JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE mr.rev_stars= 3 OR mr.rev_stars = 4
GROUP BY m.mov_year
ORDER BY m.mov_year;
```

**Output:**
| movie_year |
|------------|
|1997|

---

## Query 39

**Question:** Write a SQL query to get the reviewer name, movie title, and stars in an order that reviewer name will come first, then by movie title, and lastly by number of stars.

**SQL Query:**

```sql
SELECT r.rev_name AS reviewer_name , m.mov_title AS movie_title , mr.rev_stars
FROM
movie m JOIN movie_rating mr ON m.mov_id = mr.mov_id
		JOIN movie_reviewer r ON r.rev_id = mr.rev_id
		ORDER BY r.rev_name ASC,
				 m.mov_title ASC,
                 mr.rev_stars ASC;
```

**Output:**
| reviewer_name | movie_title | rev_stars |
|-----------------------|------------------------|-----------|
|NULL | Blade Runner | 8.2 |
| NULL| Princess Mononoke | 8.4 |
| Brandt Sponseller | Aliens | 8.4 |
| Flagrant Baronessa | Lawrence of Arabia | 8.3 |
| Hannah Steele | Donnie Darko | 8.1 |
| Jack Malvern | The Innocents | 7.9 |
| Josh Cates | Good Will Hunting | 4.0 |
| Krug Stillo | Seven Samurai | 7.7 |
| Mike Salvati | Annie Hall | 8.1 |
| Neal Wruck | Chinatown | NULL |
| Paul Monks | Boogie Nights | 3.0 |
| Richard Adams | Beyond the Sea | 6.7 |
| Righty Sock | Titanic | 7.7 |
| Righty Sock | Vertigo | 8.4 |
| Sasha Goldshtein | American Beauty | 7.0 |
| Scott LeBrun | Trainspotting | NULL |
| Simon Wright | The Usual Suspects | 8.6 |
| Victor Woeltjen | Avatar | 7.3 |
| Vincent Cadena | Slumdog Millionaire | 8.0 |

---

## Query 40

**Question:** Write a SQL query to find those movies that have at least one rating and received the most stars. Sort the result-set on movie title. Return movie title and maximum review stars.

**SQL Query:**

```sql
SELECT
    m.mov_title,
    MAX(r.rev_stars) AS max_review_stars
FROM movie m
JOIN movie_rating r
    ON m.mov_id = r.mov_id
GROUP BY m.mov_title
HAVING MAX(r.rev_stars) = (
    SELECT MAX(rev_stars)
    FROM movie_rating
)
ORDER BY m.mov_title;
```

**Output:**
| mov_title | max_review_stars |
|-----------|------------------|
|The Usual Suspects| 8.6|

---

## Query 41

**Question:** Write a SQL query to find out which movies have received ratings. Return movie title, director first name, director last name and review stars.

**SQL Query:**

```sql
SELECT m.mov_title AS movie_name , d.dir_fname , d.dir_lname, r.rev_stars AS review_stars
FROM
movie m JOIN movie_rating r ON m.mov_id = r.mov_id
		JOIN movie_direction md ON m.mov_id = md.mov_id
        JOIN director d ON d.dir_id = md.dir_id
WHERE
	r.num_o_ratings > 0;
```

**Output:**
| movie_name | dir_fname | dir_lname | review_stars |
|-----------------------|-----------|-----------------|--------------|
| Vertigo | Alfred | Hitchcock | 8.4 |
| The Innocents | Jack | Clayton | 7.9 |
| Lawrence of Arabia | David | Lean | 8.3 |
| Blade Runner | Ridley | Scott | 8.2 |
| The Usual Suspects | Bryan | Singer | 8.6 |
| Chinatown | Roman | Polanski | NULL |
| Boogie Nights | Paul | Thomas Anderson | 3.0 |
| Annie Hall | Woody | Allen | 8.1 |
| American Beauty | Sam | Mendes | 7.0 |
| Titanic | James | Cameron | 7.7 |
| Good Will Hunting | Gus | Van Sant | 4.0 |
| Trainspotting | Danny | Boyle | NULL |
| Donnie Darko | Richard | Kelly | 8.1 |
| Slumdog Millionaire | Danny | Boyle | 8.0 |
| Aliens | James | Cameron | 8.4 |
| Beyond the Sea | Kevin | Spacey | 6.7 |

---

## Query 42

**Question:** Write a SQL query to find movies in which one or more actors have acted in more than one film. Return movie title, actor first and last name, and the role.

**SQL Query:**

```sql
SELECT
    m.mov_title,
    a.act_fname,
    a.acrt_lname,
    mc.role
FROM movie_cast mc
JOIN actor a ON mc.act_id = a.act_id
JOIN movie m ON mc.mov_id = m.mov_id
JOIN movie_cast mc2 ON mc.act_id = mc2.act_id
AND mc.mov_id <> mc2.mov_id
ORDER BY a.act_fname, m.mov_title;
```

**Output:**
| mov_title | act_fname | acrt_lname | role |
|-----------|-----------|------------|------|
|American| Beauty| Kevin| Spacey| Lester Burnham|
|Beyond the Sea| Kevin| Spacey| Bobby Darin|

---

## Query 43

**Question:** Write a SQL query to find the actor whose first name is 'Claire' and last name is 'Danes'. Return director first name, last name, movie title, actor first name and last name, role.

**SQL Query:**

```sql
SELECT d.dir_fname, d.dir_lname,m.mov_title , a.act_fname , a.acrt_lname , mc.role
FROM
actor a JOIN movie_cast mc ON mc.act_id = a.act_id
		JOIN movie m ON m.mov_id = mc.mov_id
        JOIN movie_direction md on md.mov_id = mc.mov_id
        JOIN director d ON d.dir_id = md.dir_id
WHERE a.act_fname = 'Claire' AND a.acrt_lname='Danes';
```

**Output:**
| dir_fname | dir_lname | mov_title | act_fname | acrt_lname | role |
|-----------|-----------|-----------|-----------|------------|------|
| Hayao |Miyazaki |Princess Mononoke |Claire |Danes |San|

---

## Query 44

**Question:** Write a SQL query to find for actors whose films have been directed by them. Return actor first name, last name, movie title and role.

**SQL Query:**

```sql
SELECT
    a.act_fname,
    a.acrt_lname,
    m.mov_title,
    mc.role
FROM actor a
JOIN movie_cast mc ON a.act_id = mc.act_id
JOIN movie m ON mc.mov_id = m.mov_id
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON md.dir_id = d.dir_id
WHERE
    a.act_fname = d.dir_fname
    AND a.acrt_lname = d.dir_lname;
```

**Output:**
| act_fname | acrt_lname | mov_title | role |
|-----------|------------|-----------|------|
| Woody | Allen | Annie Hall | Alvy Singer |
| Kevin| Spacey| Beyond the Sea| Bobby Darin|

---

## Query 45

**Question:** Write a SQL query to find the cast list of the movie ‘Chinatown’. Return first name, last name.

**SQL Query:**

```sql
SELECT a.act_fname, a.acrt_lname
FROM
actor a JOIN movie_cast mc ON a.act_id = mc.act_id
		JOIN movie m ON mc.mov_id = m.mov_id
WHERE m.mov_title = 'Chinatown';
```

**Output:**
| act_fname | acrt_lname |
|-----------|------------|
| Jack | Nicholson |
| Christian| Bale|

---

## Query 46

**Question:** Write a SQL query to find those movies where actor’s first name is 'Harrison' and last name is 'Ford'. Return movie title.

**SQL Query:**

```sql
SELECT m.mov_title AS movie_title
FROM
movie m JOIN movie_cast mc ON m.mov_id = mc.mov_id
		JOIN actor a ON a.act_id = mc.act_id
WHERE
	a.act_fname='Harrison' AND a.acrt_lname='Ford';
```

**Output:**
| movie_title |
|-------------|
Blade Runner

---

## Query 47

**Question:** Write a SQL query to find the highest-rated movies. Return movie title, movie year, review stars and releasing country.

**SQL Query:**

```sql
SELECT
    mv.mov_title,
    mv.mov_year,
    mrt.rev_stars,
    mv.mov_rel_country
FROM movie mv
JOIN movie_rating mrt ON mv.mov_id = mrt.mov_id
WHERE mrt.rev_stars = (select max(rev_stars) from movie_rating);
```

**Output:**
| mov_title | mov_year | rev_stars | mov_rel_country |
|----------------------|----------|-----------|----------------|
| The Usual Suspects | 1995 | 8.6 | UK |

---

## Query 48

**Question:** Write a SQL query to find the highest-rated ‘Mystery Movies’. Return the title, year, and rating.

**SQL Query:**

```sql
SELECT
    m.mov_title,
    m.mov_year,
    mr.rev_stars
FROM movie m
JOIN movie_rating mr
    ON m.mov_id = mr.mov_id
JOIN movie_genres mg
    ON mg.mov_id = m.mov_id
JOIN genres g
    ON mg.gen_id = g.gen_id
WHERE g.gen_title = 'Mystery'
GROUP BY m.mov_title, m.mov_year, mr.rev_stars
HAVING mr.rev_stars = MAX(mr.rev_stars);
```

**Output:**
| mov_title | mov_year | rev_stars |
|-----------|----------|-----------|
| Vertigo |1958 |8.4

---

## Query 49

**Question:** Write a SQL query to find the years when most of the ‘Mystery Movies’ produced. Count the number of generic title and compute their average rating. Group the result set on movie release year, generic title. Return movie year, generic title, number of generic title and average rating.

**SQL Query:**

```sql
SELECT
    m.mov_year,
    g.gen_title,
    COUNT(m.mov_id) AS total_movies,
    AVG(mr.rev_stars) AS avg_rating
FROM movie m
JOIN movie_genres mg
    ON m.mov_id = mg.mov_id
JOIN genres g
    ON mg.gen_id = g.gen_id
LEFT JOIN movie_rating mr
    ON m.mov_id = mr.mov_id
WHERE g.gen_title = 'Mystery'
GROUP BY
    m.mov_year,
    g.gen_title;
```

**Output:**
| mov_year | gen_title | total_movies | avg_rating |
|----------|-----------|--------------|------------|
|1958| Mystery |1| 8.40000|
|1999| Mystery |1| NULL |
|1985| Mystery |1| NULL|

---

## Query 50

**Question:** Write a query in SQL to generate a report, which contain the fields movie title, name of the female actor, year of the movie, role, movie genres, the director, date of release, and rating of that movie.

**SQL Query:**

```sql
SELECT
    m.mov_title,
    a.act_fname,
    a.acrt_lname,
    m.mov_year,
    mc.role,
    g.gen_title,
    d.dir_fname,
    d.dir_lname,
    m.mov_dt_rel,
    mr.rev_stars AS rating
FROM movie m
JOIN movie_cast mc ON m.mov_id = mc.mov_id
JOIN actor a ON a.act_id = mc.act_id
JOIN movie_genres mg ON mg.mov_id = m.mov_id
JOIN genres g ON g.gen_id = mg.gen_id
JOIN movie_direction md ON md.mov_id = m.mov_id
JOIN director d ON d.dir_id = md.dir_id
LEFT JOIN movie_rating mr ON mr.mov_id = m.mov_id
WHERE a.act_gender = 'F';
```

**Output:**
| mov_title | act_fname | acrt_lname | mov_year | role | gen_title | dir_fname | dir_lname | mov_dt_rel | rating |
|------------------|-----------|------------|----------|--------------|-----------|-----------|-----------|------------|--------|
| The Innocents| Deborah | Kerr | 1961 | Miss Giddens | Horror | Jack | Clayton | 1962-02-19 | 7.9 |
| Eyes Wide Shut | Nicole | Kidman | 1999 | Alice Harford| Mystery | Stanley | Kubrick | NULL | NULL |
| Princess Mononoke | Claire | Danes | 1997 | San | Animation | Hayao | Miyazaki | 2001-10-19 | 8.4 |
| Aliens | Sigourney | Weaver | 1986 | Ripley | Action | James | Cameron | 1986-08-29 | 8.4 |

---


-- 1. Write a SQL query to find the name and year of the movies. Return movie title, movie release year.
SELECT 
	mov_title AS movie_title , mov_year AS movie_release_year
FROM
	movie;	
    
-- 2. 
SELECT 
	mov_year AS movie_release_year
FROM
	movie
WHERE
	mov_title = "American Beauty";

-- 3. 
SELECT 
	mov_title AS movie_title 
FROM 
	movie
WHERE 
	mov_year=1999;

-- 4.
SELECT 
	mov_title AS movie_title 
FROM
	movie
WHERE 
	mov_year < 1998;
 
 -- 5. doubt what exectly want 
SELECT 
	rev_name AS name
FROM 
	movie_reviewer
UNION
SELECT 
	mov_title AS name
FROM 
	movie;

-- 6
SELECT  
	mrw.rev_name AS reviewer_name
FROM
	movie_reviewer mrw JOIN movie_rating mrt
	ON 
	mrw.rev_id = mrt.rev_id
WHERE
	mrt.rev_stars >= 7;
    
-- 7 
SELECT 
	m.mov_title AS movie_title
FROM
	movie m JOIN movie_rating mrt
    ON
    m.mov_id = mrt.mov_id
WHERE
	mrt.num_o_ratings is null;
    
-- 8
SELECT 
	mov_title AS movie_name
FROM
	movie 
WHERE 
	mov_id IN (905, 907, 917);
    
-- 9
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
 

-- 10
SELECT
	act_id AS actor_ID
FROM
	actor
WHERE
	act_fname = 'Woody'
    AND
    acrt_lname = 'Allen';

-- 11
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
    
-- 12
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
    
-- 13
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

-- 14.
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

-- 15
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

-- 16.
SELECT mov_year AS movie_year
FROM movie 
WHERE mov_id IN 
			(SELECT mov_id 
			FROM movie_rating 
			WHERE rev_stars >= 3)
            ORDER BY mov_year ASC;
            
-- 17.
SELECT mov_title AS movie_title
FROM movie 
WHERE mov_id IN
			(SELECT mov_id 
			FROM movie_rating 
			WHERE rev_stars IS NULL);  -- or rev_stars = 0 doubt..

-- 18.
SELECT rev_name AS reviewer_name
FROM movie_reviewer
WHERE rev_id NOT IN
			(SELECT rev_id 
			FROM movie_rating ); 

-- 19.
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

-- 20
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

-- 21 doubt for single group by .
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
-- using MAX
-- select m.mov_title,
--        max(r.rev_stars) AS max_stars
-- from movie m, movie_rating r
-- where m.mov_id = r.mov_id
--   and r.rev_stars = (
--         select MAX(rev_stars)
--         from movie_rating
--         where rev_stars IS NOT NULL
--       )
-- GROUP BY m.mov_title
-- ORDER BY m.mov_title ASC;

-- 22.
SELECT mrv.rev_name AS reviewer_name
FROM movie_reviewer mrv
WHERE mrv.rev_id IN (SELECT mr.rev_id 
					FROM movie_rating mr
					WHERE mr.mov_id IN (SELECT mov_id 
										FROM movie 
										WHERE mov_title = 'American Beauty')
					);
                    
-- 23.
SELECT m.mov_title AS movie_title 
FROM movie m 
WHERE m.mov_id IN
				(SELECT mr.mov_id 
				FROM movie_rating mr
				WHERE mr.rev_id IN (SELECT mrv.rev_id 
									FROM movie_reviewer AS mrv
									WHERE mrv.rev_name = 'Paul Monks')
									);

-- 24.  Write a SQL query to find the movies with the lowest ratings. Return reviewer name, movie title, and number of stars for those movies.

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
						
-- 25. Write a SQL query to find the movies directed by 'James Cameron'. Return movie title.

SELECT mov_title AS movie_title
FROM movie WHERE mov_id IN
					(SELECT mov_id
					FROM movie_direction 
					WHERE dir_id IN (SELECT dir_id 
									FROM director 
									WHERE dir_fname = 'James' AND dir_lname = 'Cameron')
					);

-- 26  Write a query in SQL to find the movies in which one or more actors appeared in more than one film.

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

-- 27 .  Write a SQL query to find all reviewers whose ratings contain a NULL value. Return reviewer name.
SELECT mrv.rev_name AS reviewer_name
FROM
	movie_reviewer mrv 
JOIN
	movie_rating mr 
ON mrv.rev_id = mr.rev_id
WHERE mr.rev_stars IS NULL;
    
-- 28. Write a SQL query to find out who was cast in the movie 'Annie Hall'. Return actor first name, last name and role.
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

-- 29. Write a SQL query to find the director who directed a movie that featured a role in 'Eyes Wide Shut'. Return director first name, last name and movie title.

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

-- 30. Write a SQL query to find the director of a movie that cast a role as Sean Maguire. Return director first name, last name and movie title.
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

-- 31. Write a SQL query to find out which actors have not appeared in any movies between 1990 and 2000 (Begin and end values are included.). Return actor first name, last name, movie title and release year.
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

-- 32. Write a SQL query to find the directors who have directed films in a variety of genres. Group the result set on director first name, last name and generic title. Sort the result-set in ascending order by director first name and last name. Return director first name, last name and number of genres movies.
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

-- 33. Write a SQL query to find the movies with year and genres. Return movie title, movie year and generic title.
SELECT m.mov_title AS movie_title , m.mov_year AS movie_year , g.gen_title 
FROM movie m
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON g.gen_id = mg.gen_id;

-- 34 Write a SQL query to find all the movies with year, genres, and name of the director.
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
    
-- 35. Write a SQL query to find the movies released before 1st January 1989. Sort the result-set in descending order by date of release. Return movie title, release year, date of release, duration, and first and last name of the director.
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

-- 36. Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.
SELECT g.gen_title , AVG(m.mov_time) AS avg_movie_time , COUNT(m.mov_id) AS no_of_movies_in_genre
FROM genres g
JOIN movie_genres mg ON g.gen_id = mg.gen_id
JOIN movie m ON m.mov_id = mg.mov_id
GROUP BY  g.gen_title;

-- 37. Write a SQL query to find movies with the shortest duration. Return movie title, movie year, director first name, last name, actor first name, last name and role.
SELECT m.mov_title , m.mov_year , d.dir_fname,d.dir_lname , a.act_fname, a.acrt_lname , mc.role
FROM 
movie m JOIN movie_cast mc ON mc.mov_id = m.mov_id
		JOIN actor a ON a.act_id = mc.act_id
        JOIN movie_direction md ON md.mov_id = m.mov_id
        JOIN director d ON d.dir_id = md.dir_id
WHERE 
	m.mov_time <= ALL(SELECT mov_time FROM movie);

-- 38. Write a SQL query to find the years in which a movie received a rating of 3 or 4. Sort the result in increasing order on movie year.
SELECT m.mov_year AS movie_year
FROM
movie m JOIN movie_rating mr ON m.mov_id = mr.mov_id 
WHERE mr.num_o_ratings= 3 OR mr.num_o_ratings = 4 
GROUP BY m.mov_year
ORDER BY m.mov_year;

-- 39. Write a SQL query to get the reviewer name, movie title, and stars in an order that reviewer name will come first, then by movie title, and lastly by number of stars.
SELECT r.rev_name AS reviewer_name , m.mov_title AS movie_title , mr.rev_stars
FROM 
movie m JOIN movie_rating mr ON m.mov_id = mr.mov_id
		JOIN movie_reviewer r ON r.rev_id = mr.rev_id
		ORDER BY r.rev_name ASC,
				 m.mov_title ASC,
                 mr.rev_stars ASC;

-- 40. Write a SQL query to find those movies that have at least one rating and received the most stars. Sort the result-set on movie title. Return movie title and maximum review stars.
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

-- 41. Write a SQL query to find out which movies have received ratings. Return movie title, director first name, director last name and review stars.
SELECT m.mov_title AS movie_name , d.dir_fname , d.dir_lname, r.rev_stars AS review_stars
FROM 
movie m JOIN movie_rating r ON m.mov_id = r.mov_id
		JOIN movie_direction md ON m.mov_id = md.mov_id
        JOIN director d ON d.dir_id = md.dir_id
WHERE 
	r.num_o_ratings > 0;
                 
-- 42.  Write a SQL query to find movies in which one or more actors have acted in more than one film. Return movie title, actor first and last name, and the role.
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

-- 43.  Write a SQL query to find the actor whose first name is 'Claire' and last name is 'Danes'. Return director first name, last name, movie title, actor first name and last name, role.
SELECT d.dir_fname, d.dir_lname,m.mov_title , a.act_fname , a.acrt_lname , mc.role
FROM 
actor a JOIN movie_cast mc ON mc.act_id = a.act_id
		JOIN movie m ON m.mov_id = mc.mov_id
        JOIN movie_direction md on md.mov_id = mc.mov_id
        JOIN director d ON d.dir_id = md.dir_id
WHERE a.act_fname = 'Claire' AND a.acrt_lname='Danes';

-- 44.  Write a SQL query to find for actors whose films have been directed by them. Return actor first name, last name, movie title and role.
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
    
-- 45. Write a SQL query to find the cast list of the movie ‘Chinatown’. Return first name, last name.
SELECT a.act_fname, a.acrt_lname 
FROM 
actor a JOIN movie_cast mc ON a.act_id = mc.act_id
		JOIN movie m ON mc.mov_id = m.mov_id
WHERE m.mov_title = 'Chinatown';
    
-- 46. Write a SQL query to find those movies where actor’s first name is 'Harrison' and last name is 'Ford'. Return movie title.
SELECT m.mov_title AS movie_title 
FROM 
movie m JOIN movie_cast mc ON m.mov_id = mc.mov_id
		JOIN actor a ON a.act_id = mc.act_id
WHERE 
	a.act_fname='Harrison' AND a.acrt_lname='Ford';
 
-- 47. Write a SQL query to find the highest-rated movies. Return movie title, movie year, review stars and releasing country.
SELECT 
    m.mov_title,
    m.mov_year,
    r.rev_stars,
    m.mov_rel_country
FROM movie m
JOIN movie_rating r ON m.mov_id = r.mov_id
LEFT JOIN movie_rating r2 ON r.rev_stars < r2.rev_stars
WHERE r2.mov_id IS NULL;

-- 48.  Write a SQL query to find the highest-rated ‘Mystery Movies’. Return the title, year, and rating.
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

-- 49. Write a SQL query to find the years when most of the ‘Mystery Movies’ produced. Count the number of generic title and compute their average rating. Group the result set on movie release year, generic title. Return movie year, generic title, number of generic title and average rating.
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
    
-- 50. Write a query in SQL to generate a report, which contain the fields movie title, name of the female actor, year of the movie, role, movie genres, the director, date of release, and rating of that movie.

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

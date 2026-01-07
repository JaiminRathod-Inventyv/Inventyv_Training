# DBMS – Movie Database Queries

This folder contains a MySQL assignment centered on a movie database. It includes 50 SQL queries that progress from basic selection to complex joins and subqueries.

- **Files**

  - `mysql_movies.sql` – SQL script with all queries.
  - `mysql_movies_db.md` – Documentation with questions, solutions, and sample outputs.
  - `Questions.txt` – List of all 50 questions grouped by topic.

- **Query Categories:**

  1. **Basic Queries (1-10):**

   - Simple SELECT statements
   - WHERE clauses
   - UNION operations

  2. **Sub-Queries (11-26):**

   - Nested queries
   - IN, EXISTS operators

  3. **Joins (27-50):**
   - JOIN
   - Aggregate functions (COUNT, AVG, MAX, MIN)
   - GROUP BY and HAVING clauses

- **Schema at a glance**

  - `movie`, `actor`, `director`, `movie_cast`, `movie_direction`, `movie_reviewer`, `movie_rating`, `genres`, `movie_genres`.
  - Relationships cover casting, direction, reviews/ratings, and genre tagging.

- **How to use**
  1. Import or run `mysql_movies.sql` in a MySQL-compatible environment.
  2. Review `mysql_movies_db.md` to see each question, SQL answer, and output.
  3. Tweak or extend the queries for practice (e.g., add filters or new aggregates).

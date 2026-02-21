import style from "./style.module.css";

const ViewMovieList = () => {
  // モックデータ
  const movieListMockData = {
    movie_list: [
      {
        movie_id: "1",
        title: "TestMovie1",
        description: "This is a test movie",
        user_name: "TestUser",
        created_date: "2025-09-20",
        user_id: "111",
      },
      {
        movie_id: "2",
        title: "TestMovie2",
        description: "This is a test movie2",
        user_name: "hokto",
        created_date: "2024-01-01",
        user_id: "222",
      },
    ],
    total_count: 10,
    page: 1,
    count: 2,
  };

  return (
    <div className={style["view-movie-list-container"]}>
      <table className={style["view-movie-list-table"]}>
        <thead className={style["view-movie-table-header"]}>
          <tr>
            <th>映画ID</th>
            <th>映画タイトル</th>
            <th>登録者</th>
            <th>登録日付</th>
            <th>映画説明</th>
          </tr>
        </thead>
        <tbody>
          {movieListMockData.movie_list.map((movie) => (
            <tr key={movie.movie_id}>
              <td>{movie.movie_id}</td>
              <td>{movie.title}</td>
              <td>{movie.user_name}</td>
              <td>{movie.created_date}</td>
              <td>{movie.description}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};
export default ViewMovieList;

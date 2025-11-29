"use client";
import SearchMovieList from "@/components/SearchMovieList";

import style from "./style.module.css";

export const MovieList = () => {
  return (
    <div className={style["movie-list-container"]}>
      <SearchMovieList />
    </div>
  );
};
export default MovieList;

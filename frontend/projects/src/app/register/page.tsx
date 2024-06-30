"use client"
import style from "./Register.module.css";
import {useForm} from "react-hook-form";
import TextBox from "../components/form/TextBox/TextBox";
import TextBoxArea from "../components/form/TextBoxArea/TextBoxArea";

type RegisterProps={
    movieTitle: string;
    movieContents: string;
    movieMemo:string;
};
export const Register = () =>{
    const {register,handleSubmit} = useForm<RegisterProps>();
    const postRegisterForm = (formData : RegisterProps) =>{
        const{
            movieTitle,
            movieContents,
            movieMemo,
        }=formData;
        console.log("Title:",movieTitle);
        console.log("Contents:",movieContents);
        console.log("Memo:",movieMemo);
    }
    return (
        <main className={style.main}>
            <form className={style.form_group} onSubmit={handleSubmit(postRegisterForm)}>
                <TextBox title="映画タイトル" name="movie_title" register={register("movieTitle")}/>
                <TextBoxArea title="映画情報" name="movie_contents" register={register("movieContents")}/>
                <TextBoxArea title="メモ" name="movie_memo" register={register("movieMemo")}/>
                <div className={style.form_btn}>
                    <button id="register_btn" type="submit">登録</button>
                </div>
            </form>
        </main>
    )
}
export default Register;
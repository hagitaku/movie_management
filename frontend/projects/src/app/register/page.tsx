import style from "./Register.module.css";
import {useForm} from "react-hook-form";
import TextBox from "../components/form/TextBox/TextBox";
import TextBoxArea from "../components/form/TextBoxArea/TextBoxArea";

export const Register = () =>{
    async function postRegisterForm(formData : any) {
        'use server'
        var movieTitle = formData.get("movie_title");
        var movieContents = formData.get("movie_contents");
        var movieMemo = formData.get("movie_memo");
        console.log("Title:",movieTitle);
        console.log("Contents:",movieContents);
        console.log("Memo:",movieMemo);
    }
    return (
        <main className={style.main}>
            <form className={style.form_group} action={postRegisterForm}>
                <TextBox textBoxName="映画タイトル" textBoxLabel="movie_title" textBoxType="text" />
                <TextBoxArea textBoxAreaName="映画情報" textBoxAreaLabel="movie_contents" textBoxAreaRows={6} textBoxAreaCols={40}/>
                <TextBoxArea textBoxAreaName="メモ" textBoxAreaLabel="movie_memo" textBoxAreaRows={6} textBoxAreaCols={40} />
                <div className={style.form_btn}>
                    <button id="register_btn" type="submit">登録</button>
                </div>
            </form>
        </main>
    )
}
export default Register;
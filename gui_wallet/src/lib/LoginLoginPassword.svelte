<script lang="ts">
    import { Router, Route, Link, navigate } from "svelte-navigator";
    import Textfield from '@smui/textfield';
    import Button, { Label } from '@smui/button';

    function enter_main(){
        navigate("/main")
    }
</script>

<div class="login_form">
    <h1>Вход по логину и паролю</h1>
    <Textfield label="Логин" value="" />
    <Textfield label="Пароль" value="" type="password" />
    <Button variant="raised" on:click={enter_main}>Войти</Button>
    <Button color="primary" on:click={() => navigate("/")}>
        <Label>Назад</Label>
    </Button>
    
</div>

<style>
    .login_form {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>

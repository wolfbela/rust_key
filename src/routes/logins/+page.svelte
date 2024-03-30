<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    const urlParams = new URLSearchParams(window.location.search);
    const masterKey = urlParams.get("master_key");
    let logins = "";

    async function load_logins() {
        logins = await invoke<string>("load_logins", { master_key: masterKey });
        return true;
    }

    async function save_logins() {
        await invoke("write_logins_into_file", { key: masterKey });
    }

    async function add_login() {
        await invoke("create_new_login", {
            name: "name",
            username: "username",
            password: "password",
            associated_websites: "associated_websites",
        });

        await save_logins();
        await load_logins();
    }

    async function delete_login() {}
</script>

<div class="logins_page" on:close={save_logins}>
    {#await load_logins() then loads}
        {#if loads}
            {#each JSON.parse(logins) as login}
                <div class="login">
                    <div class="subpart">
                        <h4>{login.username}</h4>
                    </div>
                    <div class="subpart">
                        <h4>{login.password}</h4>
                    </div>
                    <div class="subpart">
                        <h4>{login.associated_websites}</h4>
                    </div>
                    <div
                        class="subpart"
                        style="justify-self: end; margin-right: 1rem;"
                    >
                        <button
                            style="background-color: #dd5555;"
                            on:click={delete_login}
                        ></button>
                        <button style="background-color: #dddd55;"></button>
                    </div>
                </div>
            {/each}
        {:else}
            <p style="justify-self: center; color: #a1c398; font-size: 25pt;">
                LOADING
            </p>
        {/if}
    {/await}

    <button class="add_login" on:click={add_login}>+</button>
</div>

<style>
    .logins_page {
        display: flex;
        flex-direction: column;
        overflow-y: auto;
        width: 100%;
        height: 100%;

        align-items: center;

        background-color: #fefded;
    }

    .login {
        width: 90vw;
        height: 5rem;

        border-radius: 0.4rem;
        background-color: #c6ebc5;

        display: grid;
        grid-template-columns: 1fr 1fr 1fr 1fr;

        margin-top: 0.5rem;
        margin-bottom: 0.5rem;
        transition: 0.1s;
    }

    .login:hover {
        background-color: #a1c398;
    }

    .add_login {
        position: fixed;
        display: flex;
        justify-content: center;
        align-items: center;

        bottom: 3rem;
        right: 3rem;

        font-size: 18pt;

        width: 3rem;
        height: 3rem;
        border-radius: 50%;
        border: none;

        background-color: #a1c398;
        color: #fefded;

        transition: 0.1s;
    }

    .add_login:hover {
        background-color: #fefded;
        color: #a1c398;

        box-shadow: 0px 0px 10px #a1c398;
    }

    .subpart {
        display: flex;
        flex-direction: column;
        justify-content: space-evenly;
        align-items: center;
    }

    .subpart > button {
        width: 1.5rem;
        height: 1.5rem;

        border-radius: 50%;
        border: none;
    }

    .subpart > button:hover {
        cursor: pointer;
        filter: brightness(110%);
    }
</style>

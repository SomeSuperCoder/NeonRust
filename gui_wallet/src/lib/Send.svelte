<script>
    import Button from "@smui/button"
    import Textfield from '@smui/textfield';
    let to = ""
    let fee = 1.5
    let valueTypeNumberStep = 0;
    $: result = Number(valueTypeNumberStep)+fee;
</script>

<main>
    <h1>Отправить</h1>
    <div>
        <Textfield
            label="Кому"
            variant="outlined" 
            bind:value={to}
            input$rows={1.5}
            input$cols={50}
            input$resizable={false}
            textarea
        />
        <br>
        <br>
        <Textfield
          bind:value={valueTypeNumberStep}
          label="Сколько"
          variant="outlined"
          type = "number"
          input$rows={1.5}
          input$cols={50}
          input$resizable={false}
          textarea
        />
    </div>
    {#if Number(valueTypeNumberStep) > 0}
        <h2>Коммисия: {fee}</h2>
        <h2>Итого: {result}</h2>
    {/if}
</main>

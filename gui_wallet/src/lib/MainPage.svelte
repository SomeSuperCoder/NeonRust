<script lang="ts">
    import Button from "@smui/button";
    import Fab, { Icon } from '@smui/fab';
    import TopAppBar, { Row, Section} from '@smui/top-app-bar';
    import IconButton from '@smui/icon-button';
    import Checkbox from '@smui/checkbox';
    import FormField from '@smui/form-field';
    import {navigate} from 'svelte-navigator'
    let user_addres = "0xABOBA";
    import Tab, {Label } from '@smui/tab';
    import TabBar from '@smui/tab-bar';
  
    let tabs = [
      {
        icon: '$',
        label: 'Токены',
      },
      {
        icon: 'image',
        label: 'NFT',
      },
      {
        icon: 'access_time',
        label: 'История',
      },
    ];
    let active = tabs[0];
    let balance_number = 0;
    const copy_my_text = (e: any) => {
      navigator.clipboard.writeText(e.target.innerText)
    }
    let id_person = "0x5RgO"
    import Drawer, {
      AppContent,
      Content,
      Header,
      Title,
      Subtitle,
      Scrim,
    } from '@smui/drawer';
    import List, { Item, Text, Graphic, Separator, Subheader } from '@smui/list';
  
    let open = false;
    let active_icon = 'Inbox';
  
    function setActive(value: string) {
      active_icon = value;
      open = false;
    }
</script>

<main>
    <div class="drawer-container">
      <!-- Don't include fixed={fals    <Button variant="raised">Подтвердить</Button>
e} if this is a page wide drawer.
            It adds a style for absolute positioning. -->
      <Drawer variant="modal" fixed={false} bind:open>
        <Header>
          <Title>Super Mail</Title>
          <Subtitle>It's the best fake mail app drawer.</Subtitle>
        </Header>
        <Content>
          <List>
            <Item
              href="javascript:void(0)"
              on:click={() => setActive('Inbox')}
            >
              <Graphic class="material-icons" aria-hidden="true">inbox</Graphic>
              <Text>Inbox</Text>
            </Item>
            <Item
              href="javascript:void(0)"
              on:click={() => setActive('Star')}
            >
              <Graphic class="material-icons" aria-hidden="true">star</Graphic>
              <Text>Star</Text>
            </Item>
            <Item
              href="javascript:void(0)"
              on:click={() => setActive('Sent Mail')}
            >
              <Graphic class="material-icons" aria-hidden="true">send</Graphic>
              <Text>Sent Mail</Text>
            </Item>
            <Item
              href="javascript:void(0)"
              on:click={() => setActive('Drafts')}
            >
              <Graphic class="material-icons" aria-hidden="true">drafts</Graphic>
              <Text>Drafts</Text>
            </Item>
          </List>
        </Content>
      </Drawer>
    
      <!-- Don't include fixed={false} if this is a page wide drawer.
            It adds a style for absolute positioning. -->
    </div>
    <div>
      <TopAppBar variant="static">
        <Row>
          <Section>
            <IconButton class="material-icons" on:click={() => (open = !open)}>menu</IconButton>
            <Title on:click={copy_my_text}>{id_person}</Title>
          </Section>
          <Section align="end" toolbar>
            <IconButton class="material-icons" aria-label="Download" on:click={() => navigate("/person")}>person</IconButton>
            <IconButton class="material-icons" aria-label="Print this page">settings</IconButton>
          </Section>
        </Row>
      </TopAppBar>
      <div class="neon_part">
          <h1 class="balance"><span class="balance_number">{balance_number}</span> NEON</h1>
          <Fab color="primary" on:click={() => navigate("/send")}>
              <Icon class="material-icons">send</Icon>
          </Fab>
      </div>
      <div>
        <TabBar {tabs} let:tab bind:active>
          <Tab {tab}>
            <Icon class="material-icons">{tab.icon}</Icon>
            <Label>{tab.label}</Label>
          </Tab>
        </TabBar>
      </div>
    </div>
    
</main>

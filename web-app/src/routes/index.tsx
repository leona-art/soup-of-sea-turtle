import { action, useAction } from "@solidjs/router";
import { For, Show, createResource, createSignal, onMount } from "solid-js";

type User = {
  id: string;
  name: string;
}
type Room = {
  id: string;
  name: string;
  members: User[];
}
async function createUser(name: string) {
  const response = await fetch(`http://localhost:3333/user/create/${name}`, {
    method: 'POST',
    headers:{
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({})
  });
  return await response.json() as User;
}
async function getUser(id: string) {
  const response = await fetch(`http://localhost:3333/user/${id}`);
  return await response.json() as User;
}

async function createRoom(name: string) {
  const response = await fetch(`http://localhost:3333/room/create/${name}`, {
    method: 'POST'
  });
  return await response.json() as Room;
}
async function getRoom(id: string) {
  const response = await fetch(`http://localhost:3333/room/${id}`);
  return await response.json() as User;
}
async function getRooms() {
  const response = await fetch(`http://localhost:3333/room`);
  return await response.json() as Room[];
}
async function joinRoom(roomId: string, user: User) {
  const response = await fetch(`http://localhost:3333/room/join/${roomId}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(user),
  });
  return await response.json() as User;
}


export default function Home() {
  const [name, setName] = createSignal('');
  const [user, setUser] = createSignal<User>();
  onMount(async () => {
    const id = localStorage.getItem('userId');
    if (id) {
      setUser(await getUser(id));
    }
  })
  return (
    <main>
      <input type="text" id="name" value={name()} onInput={e => setName(e.target.value)} />
      <button onClick={async () => {
        const user = await createUser(name());
        setUser(user);
        localStorage.setItem('userId', user.id);
      }}>click</button>
      <Show when={user()}>
        {user => (
          <div>
            <div>
              <div>id:{user().id}</div><br />
              <div>name:{user().name}</div>
            </div>
            <hr />
            <CreateRoom />
            <Rooms />
          </div>
        )}
      </Show>
    </main>
  );
}
function CreateRoom() {
  const [name, setName] = createSignal('');
  return (
    <div>
      <input type="text" id="name" value={name()} onInput={e => setName(e.target.value)} />
      <button onClick={async () => {
        const room = await createRoom(name());
      }}>click</button>
    </div>
  )

}
function Rooms() {
  const [data] = createResource(getRooms);
  return (
    <div style={{
      display: "grid",
      "grid-template-columns": "1fr 1fr",
      "gap": "10px"

    }}>
      <For each={data()}>
        {room => (
          <div>
            <div>id:{room.id}</div>
            <div>name:{room.name}</div>
            <For each={room.members}>
              {member => (
                <div>member:{member.name}</div>
              )}
            </For>
            <button
              onClick={async () => {
                const user = await getUser(localStorage.getItem('userId')!);
                await joinRoom(room.id, user)
                .then(e=>console.log(e));
              }
              }
            >join</button>
          </div>
        )}
      </For>
    </div>
  )
}
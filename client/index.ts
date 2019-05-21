import { prisma, User } from './generated/prisma-client'

// A `main` function so that we can use async/await
async function main() {
  // Read all users from the database and print them to the console
  const allUsers: User[] = await prisma.users()
  console.log(allUsers)

  let userList: HTMLElement = document.getElementById("user")!
  for (let user of allUsers) {
    let li: HTMLElement = document.createElement("li")
    li.textContent = `${user.name} (${user.id})`
    userList.append(li)
  }
}

main().catch(e => console.error(e))

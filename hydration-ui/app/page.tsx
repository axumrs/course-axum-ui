import PageTitle from "@/components/PageTitle";
import Link from "next/link";

export default async function Home() {
  const resp = await fetch(`${process.env.apiBaseUrl}/user`);
  const data = (await resp.json()) as Resp<User[]>;
  return (
    <>
      <PageTitle>用户列表</PageTitle>
      <table>
        <thead>
          <tr>
            <th>#</th>
            <th>用户名</th>
            <th>邮箱</th>
            <th>时间</th>
          </tr>
        </thead>
        {data && (
          <tbody>
            {data.data.map((user) => (
              <tr key={user.id}>
                <td>{user.id}</td>
                <td>
                  <Link
                    href={`/${user.id}`}
                    className="underline decoration-dotted underline-offset-4 decoration-1 hover:text-orange-600"
                  >
                    {user.username}
                  </Link>
                </td>
                <td>{user.email}</td>
                <td>{user.created_at}</td>
              </tr>
            ))}
          </tbody>
        )}
      </table>
    </>
  );
}

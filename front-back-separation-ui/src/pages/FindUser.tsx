import { useQuery } from "@tanstack/react-query";
import { Link, useParams } from "react-router-dom";
import PageTitle from "../components/PageTitle";

export default function FindUser() {
  const { id } = useParams();

  const { data } = useQuery<Resp<User>>({
    queryKey: ["find-user"],
    queryFn: () => fetch(`/api/user/${id}`).then((res) => res.json()),
  });
  return (
    <>
      {data && data.data ? (
        <>
          <PageTitle>用户 #{data.data.id}</PageTitle>
          <div className="border rounded-lg p-3 border-gray-300 space-y-4">
            <h2 className="text-xl font-semibold">{data.data.username}</h2>
            <ul className="space-y-2">
              <li>用户名：{data.data.username}</li>
              <li>邮箱：{data.data.email}</li>
              <li>注册时间：{data.data.created_at}</li>
            </ul>
            <div>
              <Link to="/" className="text-sm text-gray-600">
                返回
              </Link>
            </div>
          </div>
        </>
      ) : (
        <>
          <PageTitle>用户不存在</PageTitle>
          <div>用户 #{id} 不存在</div>
        </>
      )}
    </>
  );
}

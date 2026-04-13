import React from "react";

export default function PageTitle({ children }: { children: React.ReactNode }) {
  return <h1 className="text-2xl my-3 font-bold">{children}</h1>;
}

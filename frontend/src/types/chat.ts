export type SidebarUser = {
  id?: string | number;
  avatar: string;
  name: string;
};

export type ChatProfile = {
  avatar: string;
  name: string;
  status?: string;
};

export type ChatMessage = {
  id?: string | number;
  text: string;
  mine?: boolean;
  time?: string;
};

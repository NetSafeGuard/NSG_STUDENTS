import { Routes, Route } from "react-router-dom";

import { UpdaterPage } from "@/pages/Updater";
import { App } from "@/pages/App/App";

export const RoutesList = () => {
  return (
			<Routes>
				<Route path="/" element={<UpdaterPage />} />
				<Route path="/app" element={<App />} />
			</Routes>
		);
};

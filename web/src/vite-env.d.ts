/// <reference types="vite/client" />
interface ImportMetaEnv {
	readonly VITE_APP_DOMAIN_URL: string

	readonly VITE_APP_SSE_URL: string
	readonly VITE_APP_WS_URL: string
}

interface ImportMeta {
	readonly env: ImportMetaEnv
}

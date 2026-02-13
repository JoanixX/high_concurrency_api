// dashboard layout — Server Component
// Después acá va la validación de sesión server-side
export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="min-h-screen bg-background">
      <header className="border-b px-6 py-4">
        <h1 className="text-xl font-bold">API de Alta Concurrencia</h1>
        <p className="text-sm text-muted-foreground">Motor de Apuestas en Tiempo Real</p>
      </header>
      <div className="p-6">
        {children}
      </div>
    </div>
  );
}
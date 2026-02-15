import { useDeviceStore } from "@/stores/deviceStore";
import { useAppStore } from "@/stores/appStore";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { useNavigate } from "react-router-dom";
import { Activity, Bluetooth, Power, Settings } from "lucide-react";

export function Dashboard() {
  const navigate = useNavigate();
  const { currentDevice, deviceState, isConnected } = useDeviceStore();
  const { toggleTheme } = useAppStore();

  const getStateVariant = (state: string) => {
    switch (state) {
      case "Connected":
        return "default";
      case "Connecting":
        return "secondary";
      case "Disconnected":
        return "destructive";
      default:
        return "outline";
    }
  };

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">DG-LAB 控制器</h1>
          <p className="text-muted-foreground">设备控制与管理面板</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="icon" onClick={toggleTheme}>
            <Settings className="h-4 w-4" />
          </Button>
        </div>
      </div>

      <Separator />

      {/* Device Status Card */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle className="flex items-center gap-2">
                <Bluetooth className="h-5 w-5" />
                设备状态
              </CardTitle>
              <CardDescription>当前连接的设备信息</CardDescription>
            </div>
            <Badge variant={getStateVariant(deviceState)}>
              {deviceState}
            </Badge>
          </div>
        </CardHeader>
        <CardContent>
          {isConnected && currentDevice ? (
            <div className="space-y-2">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <p className="text-sm font-medium">设备名称</p>
                  <p className="text-sm text-muted-foreground">{currentDevice.name}</p>
                </div>
                <div>
                  <p className="text-sm font-medium">设备 ID</p>
                  <p className="text-sm text-muted-foreground font-mono">{currentDevice.id}</p>
                </div>
              </div>
              <Separator className="my-4" />
              <div className="flex gap-2">
                <Button onClick={() => navigate("/control")} className="flex-1">
                  <Power className="mr-2 h-4 w-4" />
                  功率控制
                </Button>
                <Button variant="outline" onClick={() => navigate("/scanner")} className="flex-1">
                  <Bluetooth className="mr-2 h-4 w-4" />
                  切换设备
                </Button>
              </div>
            </div>
          ) : (
            <div className="text-center py-8">
              <Activity className="mx-auto h-12 w-12 text-muted-foreground mb-4" />
              <p className="text-sm text-muted-foreground mb-4">未连接设备</p>
              <Button onClick={() => navigate("/scanner")}>
                <Bluetooth className="mr-2 h-4 w-4" />
                扫描设备
              </Button>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Quick Actions */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card className="cursor-pointer hover:bg-accent transition-colors" onClick={() => navigate("/scanner")}>
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-lg">
              <Bluetooth className="h-5 w-5" />
              设备扫描
            </CardTitle>
            <CardDescription>扫描并连接 BLE 设备</CardDescription>
          </CardHeader>
        </Card>

        <Card className="cursor-pointer hover:bg-accent transition-colors" onClick={() => navigate("/control")}>
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-lg">
              <Power className="h-5 w-5" />
              功率控制
            </CardTitle>
            <CardDescription>调节通道功率和波形</CardDescription>
          </CardHeader>
        </Card>

        <Card className="cursor-pointer hover:bg-accent transition-colors" onClick={() => navigate("/waveform")}>
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-lg">
              <Activity className="h-5 w-5" />
              波形生成
            </CardTitle>
            <CardDescription>自定义输出波形</CardDescription>
          </CardHeader>
        </Card>

        <Card className="cursor-pointer hover:bg-accent transition-colors" onClick={() => navigate("/presets")}>
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-lg">
              <Settings className="h-5 w-5" />
              预设管理
            </CardTitle>
            <CardDescription>保存和加载配置预设</CardDescription>
          </CardHeader>
        </Card>
      </div>
    </div>
  );
}

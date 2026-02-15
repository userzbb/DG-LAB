import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useDeviceStore } from "@/stores/deviceStore";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { Bluetooth, Loader2, SignalHigh, SignalMedium, SignalLow, ArrowLeft } from "lucide-react";
import type { ScannedDevice } from "@/types/device";

export function DeviceScanner() {
  const navigate = useNavigate();
  const { isScanning, scannedDevices, scanDevices, connectToDevice } = useDeviceStore();
  const [connectingId, setConnectingId] = useState<string | null>(null);

  const handleConnect = async (device: ScannedDevice) => {
    setConnectingId(device.id);
    try {
      await connectToDevice(device.id);
      navigate("/control");
    } catch (error) {
      console.error("Failed to connect:", error);
    } finally {
      setConnectingId(null);
    }
  };

  const getSignalIcon = (rssi?: number) => {
    if (!rssi) return <SignalLow className="h-4 w-4" />;
    if (rssi > -60) return <SignalHigh className="h-4 w-4 text-green-500" />;
    if (rssi > -80) return <SignalMedium className="h-4 w-4 text-yellow-500" />;
    return <SignalLow className="h-4 w-4 text-red-500" />;
  };

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" onClick={() => navigate("/")}>
          <ArrowLeft className="h-5 w-5" />
        </Button>
        <div>
          <h1 className="text-3xl font-bold tracking-tight">设备扫描</h1>
          <p className="text-muted-foreground">扫描附近的 DG-LAB 设备</p>
        </div>
      </div>

      <Separator />

      {/* Scan Control */}
      <Card>
        <CardHeader>
          <CardTitle>蓝牙扫描</CardTitle>
          <CardDescription>点击按钮开始扫描附近的 BLE 设备</CardDescription>
        </CardHeader>
        <CardContent>
          <Button onClick={scanDevices} disabled={isScanning} className="w-full">
            {isScanning ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                扫描中...
              </>
            ) : (
              <>
                <Bluetooth className="mr-2 h-4 w-4" />
                开始扫描
              </>
            )}
          </Button>
        </CardContent>
      </Card>

      {/* Device List */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-semibold">
            发现的设备 ({scannedDevices.length})
          </h2>
          {isScanning && (
            <Badge variant="secondary" className="animate-pulse">
              <Loader2 className="mr-1 h-3 w-3 animate-spin" />
              扫描中
            </Badge>
          )}
        </div>

        {scannedDevices.length === 0 ? (
          <Card>
            <CardContent className="pt-6 pb-6 text-center">
              <Bluetooth className="mx-auto h-12 w-12 text-muted-foreground mb-4" />
              <p className="text-sm text-muted-foreground">
                {isScanning ? "正在搜索设备..." : "未发现设备，请点击扫描按钮"}
              </p>
            </CardContent>
          </Card>
        ) : (
          <div className="grid gap-4">
            {scannedDevices.map((device) => (
              <Card key={device.id} className="hover:bg-accent/50 transition-colors">
                <CardContent className="pt-6">
                  <div className="flex items-center justify-between">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-2">
                        <h3 className="font-semibold">{device.name}</h3>
                        {getSignalIcon(device.rssi)}
                      </div>
                      <p className="text-sm text-muted-foreground font-mono">
                        {device.id}
                      </p>
                      {device.rssi !== undefined && (
                        <p className="text-xs text-muted-foreground mt-1">
                          信号强度: {device.rssi} dBm
                        </p>
                      )}
                    </div>
                    <Button
                      onClick={() => handleConnect(device)}
                      disabled={connectingId !== null}
                    >
                      {connectingId === device.id ? (
                        <>
                          <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                          连接中
                        </>
                      ) : (
                        "连接"
                      )}
                    </Button>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

import { useState, useEffect, useRef } from "react";
import { QRCodeSVG } from "qrcode.react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { Wifi, Loader2, CheckCircle2, XCircle, Copy, Check } from "lucide-react";
import { toast } from "@/lib/toast";
import * as api from "@/lib/api";
import type { WifiConnectResponse } from "@/types/device";

interface WifiConnectorProps {
  onConnected?: (deviceId: string) => void;
  onCancel?: () => void;
}

export function WifiConnector({ onConnected, onCancel }: WifiConnectorProps) {
  const [serverUrl, setServerUrl] = useState("");
  const [isConnecting, setIsConnecting] = useState(false);
  const [connectionInfo, setConnectionInfo] = useState<WifiConnectResponse | null>(null);
  const [isBound, setIsBound] = useState(false);
  const [copied, setCopied] = useState(false);
  const checkIntervalRef = useRef<number | null>(null);

  // 轮询检查绑定状态
  useEffect(() => {
    if (connectionInfo && !isBound) {
      checkIntervalRef.current = setInterval(async () => {
        try {
          const bound = await api.wifiCheckBinding(connectionInfo.device_id);
          if (bound) {
            setIsBound(true);
            toast.success("设备绑定成功！");
            if (checkIntervalRef.current) {
              clearInterval(checkIntervalRef.current);
            }
            // 延迟一下让用户看到成功提示
            setTimeout(() => {
              onConnected?.(connectionInfo.device_id);
            }, 1000);
          }
        } catch (error) {
          console.error("Failed to check binding status:", error);
        }
      }, 2000); // 每 2 秒检查一次

      return () => {
        if (checkIntervalRef.current) {
          clearInterval(checkIntervalRef.current);
        }
      };
    }
  }, [connectionInfo, isBound, onConnected]);

  const handleConnect = async () => {
    setIsConnecting(true);
    try {
      const response = await api.wifiConnect({
        server_url: serverUrl || undefined,
      });
      setConnectionInfo(response);
      toast.success("已连接到 WiFi 服务器");
    } catch (error) {
      console.error("WiFi connection failed:", error);
      toast.error(`WiFi 连接失败: ${error}`);
    } finally {
      setIsConnecting(false);
    }
  };

  const handleCancel = async () => {
    if (connectionInfo) {
      try {
        await api.wifiCancel(connectionInfo.device_id);
      } catch (error) {
        console.error("Failed to cancel WiFi connection:", error);
      }
    }
    if (checkIntervalRef.current) {
      clearInterval(checkIntervalRef.current);
    }
    setConnectionInfo(null);
    setIsBound(false);
    onCancel?.();
  };

  const handleCopyUrl = () => {
    if (connectionInfo?.qr_url) {
      navigator.clipboard.writeText(connectionInfo.qr_url);
      setCopied(true);
      toast.success("已复制到剪贴板");
      setTimeout(() => setCopied(false), 2000);
    }
  };

  if (connectionInfo) {
    return (
      <Card className="w-full max-w-md mx-auto">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              {isBound ? (
                <>
                  <CheckCircle2 className="h-5 w-5 text-green-500" />
                  绑定成功
                </>
              ) : (
                <>
                  <Loader2 className="h-5 w-5 animate-spin" />
                  等待设备绑定
                </>
              )}
            </CardTitle>
          </div>
          <CardDescription>
            {isBound
              ? "设备已成功绑定，正在跳转..."
              : "请使用 DG-LAB APP 扫描二维码进行绑定"}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {!isBound && (
            <>
              {/* 二维码 */}
              <div className="flex justify-center p-4 bg-white rounded-lg">
                <QRCodeSVG 
                  value={connectionInfo.qr_url} 
                  size={256}
                  level="M"
                  includeMargin={true}
                />
              </div>

              <Separator />

              {/* URL */}
              <div className="space-y-2">
                <Label>绑定链接</Label>
                <div className="flex gap-2">
                  <Input
                    value={connectionInfo.qr_url}
                    readOnly
                    className="font-mono text-xs"
                  />
                  <Button
                    variant="outline"
                    size="icon"
                    onClick={handleCopyUrl}
                  >
                    {copied ? (
                      <Check className="h-4 w-4" />
                    ) : (
                      <Copy className="h-4 w-4" />
                    )}
                  </Button>
                </div>
                <p className="text-xs text-muted-foreground">
                  也可以手动复制链接在 APP 中打开
                </p>
              </div>

              {/* 设备信息 */}
              <div className="bg-muted rounded-lg p-3 space-y-1 text-sm">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">设备名称:</span>
                  <span className="font-medium">{connectionInfo.device_name}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">设备 ID:</span>
                  <span className="font-mono text-xs">{connectionInfo.device_id.slice(0, 8)}...</span>
                </div>
              </div>
            </>
          )}

          {/* 操作按钮 */}
          <div className="flex gap-2">
            {!isBound && (
              <Button
                variant="outline"
                onClick={handleCancel}
                className="flex-1"
              >
                <XCircle className="mr-2 h-4 w-4" />
                取消
              </Button>
            )}
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className="w-full max-w-md mx-auto">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Wifi className="h-5 w-5" />
          WiFi 连接
        </CardTitle>
        <CardDescription>
          通过 WiFi 连接到 DG-LAB 设备
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* 服务器地址（可选） */}
        <div className="space-y-2">
          <Label htmlFor="server-url">服务器地址（可选）</Label>
          <Input
            id="server-url"
            placeholder="wss://ws.dungeon-lab.cn（默认）"
            value={serverUrl}
            onChange={(e) => setServerUrl(e.target.value)}
            disabled={isConnecting}
          />
          <p className="text-xs text-muted-foreground">
            留空使用官方服务器，或输入自定义服务器地址
          </p>
        </div>

        <Separator />

        {/* 连接按钮 */}
        <Button
          onClick={handleConnect}
          disabled={isConnecting}
          className="w-full"
        >
          {isConnecting ? (
            <>
              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              连接中...
            </>
          ) : (
            <>
              <Wifi className="mr-2 h-4 w-4" />
              开始连接
            </>
          )}
        </Button>

        {/* 说明 */}
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-3 space-y-2 text-sm">
          <p className="font-medium text-blue-900">连接步骤：</p>
          <ol className="list-decimal list-inside space-y-1 text-blue-800">
            <li>点击"开始连接"按钮</li>
            <li>使用 DG-LAB APP 扫描二维码</li>
            <li>在 APP 中确认绑定</li>
            <li>等待连接建立</li>
          </ol>
        </div>
      </CardContent>
    </Card>
  );
}

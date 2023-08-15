package lx.protogen.lbm;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.57.0)",
    comments = "Source: lbm.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class LubanManagerGrpc {

  private LubanManagerGrpc() {}

  public static final java.lang.String SERVICE_NAME = "lbm.LubanManager";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.NewKeygenSession,
      lx.protogen.lbm.Lbm.NewSessionResponse> getBizNewKeygenSessionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "BizNewKeygenSession",
      requestType = lx.protogen.lbm.Lbm.NewKeygenSession.class,
      responseType = lx.protogen.lbm.Lbm.NewSessionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.NewKeygenSession,
      lx.protogen.lbm.Lbm.NewSessionResponse> getBizNewKeygenSessionMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.NewKeygenSession, lx.protogen.lbm.Lbm.NewSessionResponse> getBizNewKeygenSessionMethod;
    if ((getBizNewKeygenSessionMethod = LubanManagerGrpc.getBizNewKeygenSessionMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getBizNewKeygenSessionMethod = LubanManagerGrpc.getBizNewKeygenSessionMethod) == null) {
          LubanManagerGrpc.getBizNewKeygenSessionMethod = getBizNewKeygenSessionMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.NewKeygenSession, lx.protogen.lbm.Lbm.NewSessionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "BizNewKeygenSession"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.NewKeygenSession.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.NewSessionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("BizNewKeygenSession"))
              .build();
        }
      }
    }
    return getBizNewKeygenSessionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.ProgressRequest,
      lx.protogen.lbm.Lbm.KeygenProgressPerSession> getBizPollKeygenProgressMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "BizPollKeygenProgress",
      requestType = lx.protogen.lbm.Lbm.ProgressRequest.class,
      responseType = lx.protogen.lbm.Lbm.KeygenProgressPerSession.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.ProgressRequest,
      lx.protogen.lbm.Lbm.KeygenProgressPerSession> getBizPollKeygenProgressMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.ProgressRequest, lx.protogen.lbm.Lbm.KeygenProgressPerSession> getBizPollKeygenProgressMethod;
    if ((getBizPollKeygenProgressMethod = LubanManagerGrpc.getBizPollKeygenProgressMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getBizPollKeygenProgressMethod = LubanManagerGrpc.getBizPollKeygenProgressMethod) == null) {
          LubanManagerGrpc.getBizPollKeygenProgressMethod = getBizPollKeygenProgressMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.ProgressRequest, lx.protogen.lbm.Lbm.KeygenProgressPerSession>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "BizPollKeygenProgress"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.ProgressRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.KeygenProgressPerSession.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("BizPollKeygenProgress"))
              .build();
        }
      }
    }
    return getBizPollKeygenProgressMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.NewSignSession,
      lx.protogen.lbm.Lbm.NewSessionResponse> getBizNewSignSessionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "BizNewSignSession",
      requestType = lx.protogen.lbm.Lbm.NewSignSession.class,
      responseType = lx.protogen.lbm.Lbm.NewSessionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.NewSignSession,
      lx.protogen.lbm.Lbm.NewSessionResponse> getBizNewSignSessionMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.NewSignSession, lx.protogen.lbm.Lbm.NewSessionResponse> getBizNewSignSessionMethod;
    if ((getBizNewSignSessionMethod = LubanManagerGrpc.getBizNewSignSessionMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getBizNewSignSessionMethod = LubanManagerGrpc.getBizNewSignSessionMethod) == null) {
          LubanManagerGrpc.getBizNewSignSessionMethod = getBizNewSignSessionMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.NewSignSession, lx.protogen.lbm.Lbm.NewSessionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "BizNewSignSession"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.NewSignSession.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.NewSessionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("BizNewSignSession"))
              .build();
        }
      }
    }
    return getBizNewSignSessionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.ProgressRequest,
      lx.protogen.lbm.Lbm.SignProgressPerSession> getBizPollSignProgressMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "BizPollSignProgress",
      requestType = lx.protogen.lbm.Lbm.ProgressRequest.class,
      responseType = lx.protogen.lbm.Lbm.SignProgressPerSession.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.ProgressRequest,
      lx.protogen.lbm.Lbm.SignProgressPerSession> getBizPollSignProgressMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.ProgressRequest, lx.protogen.lbm.Lbm.SignProgressPerSession> getBizPollSignProgressMethod;
    if ((getBizPollSignProgressMethod = LubanManagerGrpc.getBizPollSignProgressMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getBizPollSignProgressMethod = LubanManagerGrpc.getBizPollSignProgressMethod) == null) {
          LubanManagerGrpc.getBizPollSignProgressMethod = getBizPollSignProgressMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.ProgressRequest, lx.protogen.lbm.Lbm.SignProgressPerSession>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "BizPollSignProgress"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.ProgressRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.SignProgressPerSession.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("BizPollSignProgress"))
              .build();
        }
      }
    }
    return getBizPollSignProgressMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.PushTxRequest,
      lx.protogen.lbm.Lbm.Void> getMpcPushTxMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MpcPushTx",
      requestType = lx.protogen.lbm.Lbm.PushTxRequest.class,
      responseType = lx.protogen.lbm.Lbm.Void.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.PushTxRequest,
      lx.protogen.lbm.Lbm.Void> getMpcPushTxMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.PushTxRequest, lx.protogen.lbm.Lbm.Void> getMpcPushTxMethod;
    if ((getMpcPushTxMethod = LubanManagerGrpc.getMpcPushTxMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getMpcPushTxMethod = LubanManagerGrpc.getMpcPushTxMethod) == null) {
          LubanManagerGrpc.getMpcPushTxMethod = getMpcPushTxMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.PushTxRequest, lx.protogen.lbm.Lbm.Void>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MpcPushTx"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.PushTxRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.Void.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("MpcPushTx"))
              .build();
        }
      }
    }
    return getMpcPushTxMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.PullTxRequest,
      lx.protogen.lbm.Lbm.Tx> getMpcPullTxMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MpcPullTx",
      requestType = lx.protogen.lbm.Lbm.PullTxRequest.class,
      responseType = lx.protogen.lbm.Lbm.Tx.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.PullTxRequest,
      lx.protogen.lbm.Lbm.Tx> getMpcPullTxMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.PullTxRequest, lx.protogen.lbm.Lbm.Tx> getMpcPullTxMethod;
    if ((getMpcPullTxMethod = LubanManagerGrpc.getMpcPullTxMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getMpcPullTxMethod = LubanManagerGrpc.getMpcPullTxMethod) == null) {
          LubanManagerGrpc.getMpcPullTxMethod = getMpcPullTxMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.PullTxRequest, lx.protogen.lbm.Lbm.Tx>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MpcPullTx"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.PullTxRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.Tx.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("MpcPullTx"))
              .build();
        }
      }
    }
    return getMpcPullTxMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.KeygenTermination,
      lx.protogen.lbm.Lbm.Void> getMpcTerminateKeygenMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MpcTerminateKeygen",
      requestType = lx.protogen.lbm.Lbm.KeygenTermination.class,
      responseType = lx.protogen.lbm.Lbm.Void.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.KeygenTermination,
      lx.protogen.lbm.Lbm.Void> getMpcTerminateKeygenMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.KeygenTermination, lx.protogen.lbm.Lbm.Void> getMpcTerminateKeygenMethod;
    if ((getMpcTerminateKeygenMethod = LubanManagerGrpc.getMpcTerminateKeygenMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getMpcTerminateKeygenMethod = LubanManagerGrpc.getMpcTerminateKeygenMethod) == null) {
          LubanManagerGrpc.getMpcTerminateKeygenMethod = getMpcTerminateKeygenMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.KeygenTermination, lx.protogen.lbm.Lbm.Void>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MpcTerminateKeygen"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.KeygenTermination.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.Void.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("MpcTerminateKeygen"))
              .build();
        }
      }
    }
    return getMpcTerminateKeygenMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.SignTermination,
      lx.protogen.lbm.Lbm.Void> getMpcTerminateSignMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MpcTerminateSign",
      requestType = lx.protogen.lbm.Lbm.SignTermination.class,
      responseType = lx.protogen.lbm.Lbm.Void.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.SignTermination,
      lx.protogen.lbm.Lbm.Void> getMpcTerminateSignMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.SignTermination, lx.protogen.lbm.Lbm.Void> getMpcTerminateSignMethod;
    if ((getMpcTerminateSignMethod = LubanManagerGrpc.getMpcTerminateSignMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getMpcTerminateSignMethod = LubanManagerGrpc.getMpcTerminateSignMethod) == null) {
          LubanManagerGrpc.getMpcTerminateSignMethod = getMpcTerminateSignMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.SignTermination, lx.protogen.lbm.Lbm.Void>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MpcTerminateSign"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.SignTermination.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.Void.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("MpcTerminateSign"))
              .build();
        }
      }
    }
    return getMpcTerminateSignMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.AttendRequest,
      lx.protogen.lbm.Lbm.AttendResponse> getMpcAttendAgreeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MpcAttendAgree",
      requestType = lx.protogen.lbm.Lbm.AttendRequest.class,
      responseType = lx.protogen.lbm.Lbm.AttendResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.AttendRequest,
      lx.protogen.lbm.Lbm.AttendResponse> getMpcAttendAgreeMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.AttendRequest, lx.protogen.lbm.Lbm.AttendResponse> getMpcAttendAgreeMethod;
    if ((getMpcAttendAgreeMethod = LubanManagerGrpc.getMpcAttendAgreeMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getMpcAttendAgreeMethod = LubanManagerGrpc.getMpcAttendAgreeMethod) == null) {
          LubanManagerGrpc.getMpcAttendAgreeMethod = getMpcAttendAgreeMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.AttendRequest, lx.protogen.lbm.Lbm.AttendResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MpcAttendAgree"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.AttendRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.AttendResponse.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("MpcAttendAgree"))
              .build();
        }
      }
    }
    return getMpcAttendAgreeMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.AttendRequest,
      lx.protogen.lbm.Lbm.Void> getMpcAttendDisagreeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MpcAttendDisagree",
      requestType = lx.protogen.lbm.Lbm.AttendRequest.class,
      responseType = lx.protogen.lbm.Lbm.Void.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.AttendRequest,
      lx.protogen.lbm.Lbm.Void> getMpcAttendDisagreeMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.AttendRequest, lx.protogen.lbm.Lbm.Void> getMpcAttendDisagreeMethod;
    if ((getMpcAttendDisagreeMethod = LubanManagerGrpc.getMpcAttendDisagreeMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getMpcAttendDisagreeMethod = LubanManagerGrpc.getMpcAttendDisagreeMethod) == null) {
          LubanManagerGrpc.getMpcAttendDisagreeMethod = getMpcAttendDisagreeMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.AttendRequest, lx.protogen.lbm.Lbm.Void>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MpcAttendDisagree"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.AttendRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.Void.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("MpcAttendDisagree"))
              .build();
        }
      }
    }
    return getMpcAttendDisagreeMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.MessageToPull,
      lx.protogen.lbm.Lbm.MessagePulled> getMpcPullMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MpcPullMessage",
      requestType = lx.protogen.lbm.Lbm.MessageToPull.class,
      responseType = lx.protogen.lbm.Lbm.MessagePulled.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.MessageToPull,
      lx.protogen.lbm.Lbm.MessagePulled> getMpcPullMessageMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.MessageToPull, lx.protogen.lbm.Lbm.MessagePulled> getMpcPullMessageMethod;
    if ((getMpcPullMessageMethod = LubanManagerGrpc.getMpcPullMessageMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getMpcPullMessageMethod = LubanManagerGrpc.getMpcPullMessageMethod) == null) {
          LubanManagerGrpc.getMpcPullMessageMethod = getMpcPullMessageMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.MessageToPull, lx.protogen.lbm.Lbm.MessagePulled>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MpcPullMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.MessageToPull.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.MessagePulled.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("MpcPullMessage"))
              .build();
        }
      }
    }
    return getMpcPullMessageMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.MessageToPush,
      lx.protogen.lbm.Lbm.Void> getMpcPushMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MpcPushMessage",
      requestType = lx.protogen.lbm.Lbm.MessageToPush.class,
      responseType = lx.protogen.lbm.Lbm.Void.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.MessageToPush,
      lx.protogen.lbm.Lbm.Void> getMpcPushMessageMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbm.Lbm.MessageToPush, lx.protogen.lbm.Lbm.Void> getMpcPushMessageMethod;
    if ((getMpcPushMessageMethod = LubanManagerGrpc.getMpcPushMessageMethod) == null) {
      synchronized (LubanManagerGrpc.class) {
        if ((getMpcPushMessageMethod = LubanManagerGrpc.getMpcPushMessageMethod) == null) {
          LubanManagerGrpc.getMpcPushMessageMethod = getMpcPushMessageMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbm.Lbm.MessageToPush, lx.protogen.lbm.Lbm.Void>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MpcPushMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.MessageToPush.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.Void.getDefaultInstance()))
              .setSchemaDescriptor(new LubanManagerMethodDescriptorSupplier("MpcPushMessage"))
              .build();
        }
      }
    }
    return getMpcPushMessageMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static LubanManagerStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<LubanManagerStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<LubanManagerStub>() {
        @java.lang.Override
        public LubanManagerStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new LubanManagerStub(channel, callOptions);
        }
      };
    return LubanManagerStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static LubanManagerBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<LubanManagerBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<LubanManagerBlockingStub>() {
        @java.lang.Override
        public LubanManagerBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new LubanManagerBlockingStub(channel, callOptions);
        }
      };
    return LubanManagerBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static LubanManagerFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<LubanManagerFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<LubanManagerFutureStub>() {
        @java.lang.Override
        public LubanManagerFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new LubanManagerFutureStub(channel, callOptions);
        }
      };
    return LubanManagerFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void bizNewKeygenSession(lx.protogen.lbm.Lbm.NewKeygenSession request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.NewSessionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getBizNewKeygenSessionMethod(), responseObserver);
    }

    /**
     */
    default void bizPollKeygenProgress(lx.protogen.lbm.Lbm.ProgressRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.KeygenProgressPerSession> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getBizPollKeygenProgressMethod(), responseObserver);
    }

    /**
     */
    default void bizNewSignSession(lx.protogen.lbm.Lbm.NewSignSession request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.NewSessionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getBizNewSignSessionMethod(), responseObserver);
    }

    /**
     */
    default void bizPollSignProgress(lx.protogen.lbm.Lbm.ProgressRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.SignProgressPerSession> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getBizPollSignProgressMethod(), responseObserver);
    }

    /**
     */
    default void mpcPushTx(lx.protogen.lbm.Lbm.PushTxRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMpcPushTxMethod(), responseObserver);
    }

    /**
     */
    default void mpcPullTx(lx.protogen.lbm.Lbm.PullTxRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Tx> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMpcPullTxMethod(), responseObserver);
    }

    /**
     */
    default void mpcTerminateKeygen(lx.protogen.lbm.Lbm.KeygenTermination request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMpcTerminateKeygenMethod(), responseObserver);
    }

    /**
     */
    default void mpcTerminateSign(lx.protogen.lbm.Lbm.SignTermination request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMpcTerminateSignMethod(), responseObserver);
    }

    /**
     */
    default void mpcAttendAgree(lx.protogen.lbm.Lbm.AttendRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.AttendResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMpcAttendAgreeMethod(), responseObserver);
    }

    /**
     */
    default void mpcAttendDisagree(lx.protogen.lbm.Lbm.AttendRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMpcAttendDisagreeMethod(), responseObserver);
    }

    /**
     */
    default void mpcPullMessage(lx.protogen.lbm.Lbm.MessageToPull request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.MessagePulled> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMpcPullMessageMethod(), responseObserver);
    }

    /**
     */
    default void mpcPushMessage(lx.protogen.lbm.Lbm.MessageToPush request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMpcPushMessageMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service LubanManager.
   */
  public static abstract class LubanManagerImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return LubanManagerGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service LubanManager.
   */
  public static final class LubanManagerStub
      extends io.grpc.stub.AbstractAsyncStub<LubanManagerStub> {
    private LubanManagerStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected LubanManagerStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new LubanManagerStub(channel, callOptions);
    }

    /**
     */
    public void bizNewKeygenSession(lx.protogen.lbm.Lbm.NewKeygenSession request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.NewSessionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getBizNewKeygenSessionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void bizPollKeygenProgress(lx.protogen.lbm.Lbm.ProgressRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.KeygenProgressPerSession> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getBizPollKeygenProgressMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void bizNewSignSession(lx.protogen.lbm.Lbm.NewSignSession request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.NewSessionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getBizNewSignSessionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void bizPollSignProgress(lx.protogen.lbm.Lbm.ProgressRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.SignProgressPerSession> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getBizPollSignProgressMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void mpcPushTx(lx.protogen.lbm.Lbm.PushTxRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMpcPushTxMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void mpcPullTx(lx.protogen.lbm.Lbm.PullTxRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Tx> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMpcPullTxMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void mpcTerminateKeygen(lx.protogen.lbm.Lbm.KeygenTermination request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMpcTerminateKeygenMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void mpcTerminateSign(lx.protogen.lbm.Lbm.SignTermination request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMpcTerminateSignMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void mpcAttendAgree(lx.protogen.lbm.Lbm.AttendRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.AttendResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMpcAttendAgreeMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void mpcAttendDisagree(lx.protogen.lbm.Lbm.AttendRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMpcAttendDisagreeMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void mpcPullMessage(lx.protogen.lbm.Lbm.MessageToPull request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.MessagePulled> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMpcPullMessageMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void mpcPushMessage(lx.protogen.lbm.Lbm.MessageToPush request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMpcPushMessageMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service LubanManager.
   */
  public static final class LubanManagerBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<LubanManagerBlockingStub> {
    private LubanManagerBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected LubanManagerBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new LubanManagerBlockingStub(channel, callOptions);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.NewSessionResponse bizNewKeygenSession(lx.protogen.lbm.Lbm.NewKeygenSession request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBizNewKeygenSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.KeygenProgressPerSession bizPollKeygenProgress(lx.protogen.lbm.Lbm.ProgressRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBizPollKeygenProgressMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.NewSessionResponse bizNewSignSession(lx.protogen.lbm.Lbm.NewSignSession request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBizNewSignSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.SignProgressPerSession bizPollSignProgress(lx.protogen.lbm.Lbm.ProgressRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBizPollSignProgressMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.Void mpcPushTx(lx.protogen.lbm.Lbm.PushTxRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMpcPushTxMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.Tx mpcPullTx(lx.protogen.lbm.Lbm.PullTxRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMpcPullTxMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.Void mpcTerminateKeygen(lx.protogen.lbm.Lbm.KeygenTermination request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMpcTerminateKeygenMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.Void mpcTerminateSign(lx.protogen.lbm.Lbm.SignTermination request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMpcTerminateSignMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.AttendResponse mpcAttendAgree(lx.protogen.lbm.Lbm.AttendRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMpcAttendAgreeMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.Void mpcAttendDisagree(lx.protogen.lbm.Lbm.AttendRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMpcAttendDisagreeMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.MessagePulled mpcPullMessage(lx.protogen.lbm.Lbm.MessageToPull request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMpcPullMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.Void mpcPushMessage(lx.protogen.lbm.Lbm.MessageToPush request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMpcPushMessageMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service LubanManager.
   */
  public static final class LubanManagerFutureStub
      extends io.grpc.stub.AbstractFutureStub<LubanManagerFutureStub> {
    private LubanManagerFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected LubanManagerFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new LubanManagerFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.NewSessionResponse> bizNewKeygenSession(
        lx.protogen.lbm.Lbm.NewKeygenSession request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getBizNewKeygenSessionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.KeygenProgressPerSession> bizPollKeygenProgress(
        lx.protogen.lbm.Lbm.ProgressRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getBizPollKeygenProgressMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.NewSessionResponse> bizNewSignSession(
        lx.protogen.lbm.Lbm.NewSignSession request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getBizNewSignSessionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.SignProgressPerSession> bizPollSignProgress(
        lx.protogen.lbm.Lbm.ProgressRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getBizPollSignProgressMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.Void> mpcPushTx(
        lx.protogen.lbm.Lbm.PushTxRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMpcPushTxMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.Tx> mpcPullTx(
        lx.protogen.lbm.Lbm.PullTxRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMpcPullTxMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.Void> mpcTerminateKeygen(
        lx.protogen.lbm.Lbm.KeygenTermination request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMpcTerminateKeygenMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.Void> mpcTerminateSign(
        lx.protogen.lbm.Lbm.SignTermination request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMpcTerminateSignMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.AttendResponse> mpcAttendAgree(
        lx.protogen.lbm.Lbm.AttendRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMpcAttendAgreeMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.Void> mpcAttendDisagree(
        lx.protogen.lbm.Lbm.AttendRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMpcAttendDisagreeMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.MessagePulled> mpcPullMessage(
        lx.protogen.lbm.Lbm.MessageToPull request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMpcPullMessageMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.Void> mpcPushMessage(
        lx.protogen.lbm.Lbm.MessageToPush request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMpcPushMessageMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_BIZ_NEW_KEYGEN_SESSION = 0;
  private static final int METHODID_BIZ_POLL_KEYGEN_PROGRESS = 1;
  private static final int METHODID_BIZ_NEW_SIGN_SESSION = 2;
  private static final int METHODID_BIZ_POLL_SIGN_PROGRESS = 3;
  private static final int METHODID_MPC_PUSH_TX = 4;
  private static final int METHODID_MPC_PULL_TX = 5;
  private static final int METHODID_MPC_TERMINATE_KEYGEN = 6;
  private static final int METHODID_MPC_TERMINATE_SIGN = 7;
  private static final int METHODID_MPC_ATTEND_AGREE = 8;
  private static final int METHODID_MPC_ATTEND_DISAGREE = 9;
  private static final int METHODID_MPC_PULL_MESSAGE = 10;
  private static final int METHODID_MPC_PUSH_MESSAGE = 11;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final AsyncService serviceImpl;
    private final int methodId;

    MethodHandlers(AsyncService serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_BIZ_NEW_KEYGEN_SESSION:
          serviceImpl.bizNewKeygenSession((lx.protogen.lbm.Lbm.NewKeygenSession) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.NewSessionResponse>) responseObserver);
          break;
        case METHODID_BIZ_POLL_KEYGEN_PROGRESS:
          serviceImpl.bizPollKeygenProgress((lx.protogen.lbm.Lbm.ProgressRequest) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.KeygenProgressPerSession>) responseObserver);
          break;
        case METHODID_BIZ_NEW_SIGN_SESSION:
          serviceImpl.bizNewSignSession((lx.protogen.lbm.Lbm.NewSignSession) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.NewSessionResponse>) responseObserver);
          break;
        case METHODID_BIZ_POLL_SIGN_PROGRESS:
          serviceImpl.bizPollSignProgress((lx.protogen.lbm.Lbm.ProgressRequest) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.SignProgressPerSession>) responseObserver);
          break;
        case METHODID_MPC_PUSH_TX:
          serviceImpl.mpcPushTx((lx.protogen.lbm.Lbm.PushTxRequest) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void>) responseObserver);
          break;
        case METHODID_MPC_PULL_TX:
          serviceImpl.mpcPullTx((lx.protogen.lbm.Lbm.PullTxRequest) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Tx>) responseObserver);
          break;
        case METHODID_MPC_TERMINATE_KEYGEN:
          serviceImpl.mpcTerminateKeygen((lx.protogen.lbm.Lbm.KeygenTermination) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void>) responseObserver);
          break;
        case METHODID_MPC_TERMINATE_SIGN:
          serviceImpl.mpcTerminateSign((lx.protogen.lbm.Lbm.SignTermination) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void>) responseObserver);
          break;
        case METHODID_MPC_ATTEND_AGREE:
          serviceImpl.mpcAttendAgree((lx.protogen.lbm.Lbm.AttendRequest) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.AttendResponse>) responseObserver);
          break;
        case METHODID_MPC_ATTEND_DISAGREE:
          serviceImpl.mpcAttendDisagree((lx.protogen.lbm.Lbm.AttendRequest) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void>) responseObserver);
          break;
        case METHODID_MPC_PULL_MESSAGE:
          serviceImpl.mpcPullMessage((lx.protogen.lbm.Lbm.MessageToPull) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.MessagePulled>) responseObserver);
          break;
        case METHODID_MPC_PUSH_MESSAGE:
          serviceImpl.mpcPushMessage((lx.protogen.lbm.Lbm.MessageToPush) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void>) responseObserver);
          break;
        default:
          throw new AssertionError();
      }
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public io.grpc.stub.StreamObserver<Req> invoke(
        io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getBizNewKeygenSessionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.NewKeygenSession,
              lx.protogen.lbm.Lbm.NewSessionResponse>(
                service, METHODID_BIZ_NEW_KEYGEN_SESSION)))
        .addMethod(
          getBizPollKeygenProgressMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.ProgressRequest,
              lx.protogen.lbm.Lbm.KeygenProgressPerSession>(
                service, METHODID_BIZ_POLL_KEYGEN_PROGRESS)))
        .addMethod(
          getBizNewSignSessionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.NewSignSession,
              lx.protogen.lbm.Lbm.NewSessionResponse>(
                service, METHODID_BIZ_NEW_SIGN_SESSION)))
        .addMethod(
          getBizPollSignProgressMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.ProgressRequest,
              lx.protogen.lbm.Lbm.SignProgressPerSession>(
                service, METHODID_BIZ_POLL_SIGN_PROGRESS)))
        .addMethod(
          getMpcPushTxMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.PushTxRequest,
              lx.protogen.lbm.Lbm.Void>(
                service, METHODID_MPC_PUSH_TX)))
        .addMethod(
          getMpcPullTxMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.PullTxRequest,
              lx.protogen.lbm.Lbm.Tx>(
                service, METHODID_MPC_PULL_TX)))
        .addMethod(
          getMpcTerminateKeygenMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.KeygenTermination,
              lx.protogen.lbm.Lbm.Void>(
                service, METHODID_MPC_TERMINATE_KEYGEN)))
        .addMethod(
          getMpcTerminateSignMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.SignTermination,
              lx.protogen.lbm.Lbm.Void>(
                service, METHODID_MPC_TERMINATE_SIGN)))
        .addMethod(
          getMpcAttendAgreeMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.AttendRequest,
              lx.protogen.lbm.Lbm.AttendResponse>(
                service, METHODID_MPC_ATTEND_AGREE)))
        .addMethod(
          getMpcAttendDisagreeMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.AttendRequest,
              lx.protogen.lbm.Lbm.Void>(
                service, METHODID_MPC_ATTEND_DISAGREE)))
        .addMethod(
          getMpcPullMessageMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.MessageToPull,
              lx.protogen.lbm.Lbm.MessagePulled>(
                service, METHODID_MPC_PULL_MESSAGE)))
        .addMethod(
          getMpcPushMessageMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbm.Lbm.MessageToPush,
              lx.protogen.lbm.Lbm.Void>(
                service, METHODID_MPC_PUSH_MESSAGE)))
        .build();
  }

  private static abstract class LubanManagerBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    LubanManagerBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return lx.protogen.lbm.Lbm.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("LubanManager");
    }
  }

  private static final class LubanManagerFileDescriptorSupplier
      extends LubanManagerBaseDescriptorSupplier {
    LubanManagerFileDescriptorSupplier() {}
  }

  private static final class LubanManagerMethodDescriptorSupplier
      extends LubanManagerBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    LubanManagerMethodDescriptorSupplier(java.lang.String methodName) {
      this.methodName = methodName;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.MethodDescriptor getMethodDescriptor() {
      return getServiceDescriptor().findMethodByName(methodName);
    }
  }

  private static volatile io.grpc.ServiceDescriptor serviceDescriptor;

  public static io.grpc.ServiceDescriptor getServiceDescriptor() {
    io.grpc.ServiceDescriptor result = serviceDescriptor;
    if (result == null) {
      synchronized (LubanManagerGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new LubanManagerFileDescriptorSupplier())
              .addMethod(getBizNewKeygenSessionMethod())
              .addMethod(getBizPollKeygenProgressMethod())
              .addMethod(getBizNewSignSessionMethod())
              .addMethod(getBizPollSignProgressMethod())
              .addMethod(getMpcPushTxMethod())
              .addMethod(getMpcPullTxMethod())
              .addMethod(getMpcTerminateKeygenMethod())
              .addMethod(getMpcTerminateSignMethod())
              .addMethod(getMpcAttendAgreeMethod())
              .addMethod(getMpcAttendDisagreeMethod())
              .addMethod(getMpcPullMessageMethod())
              .addMethod(getMpcPushMessageMethod())
              .build();
        }
      }
    }
    return result;
  }
}

package lx.protogen.lbcs;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.57.0)",
    comments = "Source: lbcs.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class LubanClientGrpc {

  private LubanClientGrpc() {}

  public static final java.lang.String SERVICE_NAME = "lbcs.LubanClient";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.BasicMpcRequest,
      lx.protogen.lbcs.Lbcs.MpcProgress> getClientKeygenMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ClientKeygen",
      requestType = lx.protogen.lbcs.Lbcs.BasicMpcRequest.class,
      responseType = lx.protogen.lbcs.Lbcs.MpcProgress.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.BasicMpcRequest,
      lx.protogen.lbcs.Lbcs.MpcProgress> getClientKeygenMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.BasicMpcRequest, lx.protogen.lbcs.Lbcs.MpcProgress> getClientKeygenMethod;
    if ((getClientKeygenMethod = LubanClientGrpc.getClientKeygenMethod) == null) {
      synchronized (LubanClientGrpc.class) {
        if ((getClientKeygenMethod = LubanClientGrpc.getClientKeygenMethod) == null) {
          LubanClientGrpc.getClientKeygenMethod = getClientKeygenMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbcs.Lbcs.BasicMpcRequest, lx.protogen.lbcs.Lbcs.MpcProgress>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ClientKeygen"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbcs.Lbcs.BasicMpcRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbcs.Lbcs.MpcProgress.getDefaultInstance()))
              .setSchemaDescriptor(new LubanClientMethodDescriptorSupplier("ClientKeygen"))
              .build();
        }
      }
    }
    return getClientKeygenMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.KeySharingRequest,
      lx.protogen.lbcs.Lbcs.MpcProgress> getClientShareRootKeyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ClientShareRootKey",
      requestType = lx.protogen.lbcs.Lbcs.KeySharingRequest.class,
      responseType = lx.protogen.lbcs.Lbcs.MpcProgress.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.KeySharingRequest,
      lx.protogen.lbcs.Lbcs.MpcProgress> getClientShareRootKeyMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.KeySharingRequest, lx.protogen.lbcs.Lbcs.MpcProgress> getClientShareRootKeyMethod;
    if ((getClientShareRootKeyMethod = LubanClientGrpc.getClientShareRootKeyMethod) == null) {
      synchronized (LubanClientGrpc.class) {
        if ((getClientShareRootKeyMethod = LubanClientGrpc.getClientShareRootKeyMethod) == null) {
          LubanClientGrpc.getClientShareRootKeyMethod = getClientShareRootKeyMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbcs.Lbcs.KeySharingRequest, lx.protogen.lbcs.Lbcs.MpcProgress>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ClientShareRootKey"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbcs.Lbcs.KeySharingRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbcs.Lbcs.MpcProgress.getDefaultInstance()))
              .setSchemaDescriptor(new LubanClientMethodDescriptorSupplier("ClientShareRootKey"))
              .build();
        }
      }
    }
    return getClientShareRootKeyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.KeyRescueRequest,
      lx.protogen.lbcs.Lbcs.MpcProgress> getClientRescueKeyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ClientRescueKey",
      requestType = lx.protogen.lbcs.Lbcs.KeyRescueRequest.class,
      responseType = lx.protogen.lbcs.Lbcs.MpcProgress.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.KeyRescueRequest,
      lx.protogen.lbcs.Lbcs.MpcProgress> getClientRescueKeyMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.KeyRescueRequest, lx.protogen.lbcs.Lbcs.MpcProgress> getClientRescueKeyMethod;
    if ((getClientRescueKeyMethod = LubanClientGrpc.getClientRescueKeyMethod) == null) {
      synchronized (LubanClientGrpc.class) {
        if ((getClientRescueKeyMethod = LubanClientGrpc.getClientRescueKeyMethod) == null) {
          LubanClientGrpc.getClientRescueKeyMethod = getClientRescueKeyMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbcs.Lbcs.KeyRescueRequest, lx.protogen.lbcs.Lbcs.MpcProgress>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ClientRescueKey"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbcs.Lbcs.KeyRescueRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbcs.Lbcs.MpcProgress.getDefaultInstance()))
              .setSchemaDescriptor(new LubanClientMethodDescriptorSupplier("ClientRescueKey"))
              .build();
        }
      }
    }
    return getClientRescueKeyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.BasicMpcRequest,
      lx.protogen.lbcs.Lbcs.MpcProgress> getClientSignMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ClientSign",
      requestType = lx.protogen.lbcs.Lbcs.BasicMpcRequest.class,
      responseType = lx.protogen.lbcs.Lbcs.MpcProgress.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.BasicMpcRequest,
      lx.protogen.lbcs.Lbcs.MpcProgress> getClientSignMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.BasicMpcRequest, lx.protogen.lbcs.Lbcs.MpcProgress> getClientSignMethod;
    if ((getClientSignMethod = LubanClientGrpc.getClientSignMethod) == null) {
      synchronized (LubanClientGrpc.class) {
        if ((getClientSignMethod = LubanClientGrpc.getClientSignMethod) == null) {
          LubanClientGrpc.getClientSignMethod = getClientSignMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbcs.Lbcs.BasicMpcRequest, lx.protogen.lbcs.Lbcs.MpcProgress>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ClientSign"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbcs.Lbcs.BasicMpcRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbcs.Lbcs.MpcProgress.getDefaultInstance()))
              .setSchemaDescriptor(new LubanClientMethodDescriptorSupplier("ClientSign"))
              .build();
        }
      }
    }
    return getClientSignMethod;
  }

  private static volatile io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.BasicMpcRequest,
      lx.protogen.lbm.Lbm.Void> getClientDisagreeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ClientDisagree",
      requestType = lx.protogen.lbcs.Lbcs.BasicMpcRequest.class,
      responseType = lx.protogen.lbm.Lbm.Void.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.BasicMpcRequest,
      lx.protogen.lbm.Lbm.Void> getClientDisagreeMethod() {
    io.grpc.MethodDescriptor<lx.protogen.lbcs.Lbcs.BasicMpcRequest, lx.protogen.lbm.Lbm.Void> getClientDisagreeMethod;
    if ((getClientDisagreeMethod = LubanClientGrpc.getClientDisagreeMethod) == null) {
      synchronized (LubanClientGrpc.class) {
        if ((getClientDisagreeMethod = LubanClientGrpc.getClientDisagreeMethod) == null) {
          LubanClientGrpc.getClientDisagreeMethod = getClientDisagreeMethod =
              io.grpc.MethodDescriptor.<lx.protogen.lbcs.Lbcs.BasicMpcRequest, lx.protogen.lbm.Lbm.Void>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ClientDisagree"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbcs.Lbcs.BasicMpcRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  lx.protogen.lbm.Lbm.Void.getDefaultInstance()))
              .setSchemaDescriptor(new LubanClientMethodDescriptorSupplier("ClientDisagree"))
              .build();
        }
      }
    }
    return getClientDisagreeMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static LubanClientStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<LubanClientStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<LubanClientStub>() {
        @java.lang.Override
        public LubanClientStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new LubanClientStub(channel, callOptions);
        }
      };
    return LubanClientStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static LubanClientBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<LubanClientBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<LubanClientBlockingStub>() {
        @java.lang.Override
        public LubanClientBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new LubanClientBlockingStub(channel, callOptions);
        }
      };
    return LubanClientBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static LubanClientFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<LubanClientFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<LubanClientFutureStub>() {
        @java.lang.Override
        public LubanClientFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new LubanClientFutureStub(channel, callOptions);
        }
      };
    return LubanClientFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void clientKeygen(lx.protogen.lbcs.Lbcs.BasicMpcRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getClientKeygenMethod(), responseObserver);
    }

    /**
     */
    default void clientShareRootKey(lx.protogen.lbcs.Lbcs.KeySharingRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getClientShareRootKeyMethod(), responseObserver);
    }

    /**
     */
    default void clientRescueKey(lx.protogen.lbcs.Lbcs.KeyRescueRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getClientRescueKeyMethod(), responseObserver);
    }

    /**
     */
    default void clientSign(lx.protogen.lbcs.Lbcs.BasicMpcRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getClientSignMethod(), responseObserver);
    }

    /**
     */
    default void clientDisagree(lx.protogen.lbcs.Lbcs.BasicMpcRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getClientDisagreeMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service LubanClient.
   */
  public static abstract class LubanClientImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return LubanClientGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service LubanClient.
   */
  public static final class LubanClientStub
      extends io.grpc.stub.AbstractAsyncStub<LubanClientStub> {
    private LubanClientStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected LubanClientStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new LubanClientStub(channel, callOptions);
    }

    /**
     */
    public void clientKeygen(lx.protogen.lbcs.Lbcs.BasicMpcRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getClientKeygenMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void clientShareRootKey(lx.protogen.lbcs.Lbcs.KeySharingRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getClientShareRootKeyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void clientRescueKey(lx.protogen.lbcs.Lbcs.KeyRescueRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getClientRescueKeyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void clientSign(lx.protogen.lbcs.Lbcs.BasicMpcRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getClientSignMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void clientDisagree(lx.protogen.lbcs.Lbcs.BasicMpcRequest request,
        io.grpc.stub.StreamObserver<lx.protogen.lbm.Lbm.Void> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getClientDisagreeMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service LubanClient.
   */
  public static final class LubanClientBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<LubanClientBlockingStub> {
    private LubanClientBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected LubanClientBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new LubanClientBlockingStub(channel, callOptions);
    }

    /**
     */
    public java.util.Iterator<lx.protogen.lbcs.Lbcs.MpcProgress> clientKeygen(
        lx.protogen.lbcs.Lbcs.BasicMpcRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getClientKeygenMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<lx.protogen.lbcs.Lbcs.MpcProgress> clientShareRootKey(
        lx.protogen.lbcs.Lbcs.KeySharingRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getClientShareRootKeyMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<lx.protogen.lbcs.Lbcs.MpcProgress> clientRescueKey(
        lx.protogen.lbcs.Lbcs.KeyRescueRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getClientRescueKeyMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<lx.protogen.lbcs.Lbcs.MpcProgress> clientSign(
        lx.protogen.lbcs.Lbcs.BasicMpcRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getClientSignMethod(), getCallOptions(), request);
    }

    /**
     */
    public lx.protogen.lbm.Lbm.Void clientDisagree(lx.protogen.lbcs.Lbcs.BasicMpcRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getClientDisagreeMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service LubanClient.
   */
  public static final class LubanClientFutureStub
      extends io.grpc.stub.AbstractFutureStub<LubanClientFutureStub> {
    private LubanClientFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected LubanClientFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new LubanClientFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<lx.protogen.lbm.Lbm.Void> clientDisagree(
        lx.protogen.lbcs.Lbcs.BasicMpcRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getClientDisagreeMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CLIENT_KEYGEN = 0;
  private static final int METHODID_CLIENT_SHARE_ROOT_KEY = 1;
  private static final int METHODID_CLIENT_RESCUE_KEY = 2;
  private static final int METHODID_CLIENT_SIGN = 3;
  private static final int METHODID_CLIENT_DISAGREE = 4;

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
        case METHODID_CLIENT_KEYGEN:
          serviceImpl.clientKeygen((lx.protogen.lbcs.Lbcs.BasicMpcRequest) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress>) responseObserver);
          break;
        case METHODID_CLIENT_SHARE_ROOT_KEY:
          serviceImpl.clientShareRootKey((lx.protogen.lbcs.Lbcs.KeySharingRequest) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress>) responseObserver);
          break;
        case METHODID_CLIENT_RESCUE_KEY:
          serviceImpl.clientRescueKey((lx.protogen.lbcs.Lbcs.KeyRescueRequest) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress>) responseObserver);
          break;
        case METHODID_CLIENT_SIGN:
          serviceImpl.clientSign((lx.protogen.lbcs.Lbcs.BasicMpcRequest) request,
              (io.grpc.stub.StreamObserver<lx.protogen.lbcs.Lbcs.MpcProgress>) responseObserver);
          break;
        case METHODID_CLIENT_DISAGREE:
          serviceImpl.clientDisagree((lx.protogen.lbcs.Lbcs.BasicMpcRequest) request,
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
          getClientKeygenMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              lx.protogen.lbcs.Lbcs.BasicMpcRequest,
              lx.protogen.lbcs.Lbcs.MpcProgress>(
                service, METHODID_CLIENT_KEYGEN)))
        .addMethod(
          getClientShareRootKeyMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              lx.protogen.lbcs.Lbcs.KeySharingRequest,
              lx.protogen.lbcs.Lbcs.MpcProgress>(
                service, METHODID_CLIENT_SHARE_ROOT_KEY)))
        .addMethod(
          getClientRescueKeyMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              lx.protogen.lbcs.Lbcs.KeyRescueRequest,
              lx.protogen.lbcs.Lbcs.MpcProgress>(
                service, METHODID_CLIENT_RESCUE_KEY)))
        .addMethod(
          getClientSignMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              lx.protogen.lbcs.Lbcs.BasicMpcRequest,
              lx.protogen.lbcs.Lbcs.MpcProgress>(
                service, METHODID_CLIENT_SIGN)))
        .addMethod(
          getClientDisagreeMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              lx.protogen.lbcs.Lbcs.BasicMpcRequest,
              lx.protogen.lbm.Lbm.Void>(
                service, METHODID_CLIENT_DISAGREE)))
        .build();
  }

  private static abstract class LubanClientBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    LubanClientBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return lx.protogen.lbcs.Lbcs.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("LubanClient");
    }
  }

  private static final class LubanClientFileDescriptorSupplier
      extends LubanClientBaseDescriptorSupplier {
    LubanClientFileDescriptorSupplier() {}
  }

  private static final class LubanClientMethodDescriptorSupplier
      extends LubanClientBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    LubanClientMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (LubanClientGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new LubanClientFileDescriptorSupplier())
              .addMethod(getClientKeygenMethod())
              .addMethod(getClientShareRootKeyMethod())
              .addMethod(getClientRescueKeyMethod())
              .addMethod(getClientSignMethod())
              .addMethod(getClientDisagreeMethod())
              .build();
        }
      }
    }
    return result;
  }
}

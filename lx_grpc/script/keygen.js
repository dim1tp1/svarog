const grpc = require("@grpc/grpc-js");
const loader = require("@grpc/proto-loader");
console.log(__dirname);
loader.load([__dirname + "/../proto/gg18.proto", __dirname + "/../proto/lbm.proto", __dirname + "/../proto/lbcs.proto"], {
    keepCase: true,
    longs: String,
    enume: String,
    defaults: true,
    oneofs: true,
}).then((packageDefinition) =>{
    // console.log(packageDefinition);
    const package = grpc.loadPackageDefinition(packageDefinition);

    /**@var ServiceClientConstructor client */
    const client = new package.lbm.LubanManager("117.68.118.39:9000", grpc.credentials.createInsecure());
    const request = {
        owners: ["100001"],
        min_num_owners: 1
    };

    client.BizNewKeygenSession(request, (e, res) => {
        console.log(e, "------------------ error ----------------", res);
        const c = new package.lbcs.LubanClient("117.68.118.39:9001", grpc.credentials.createInsecure());
        const iterator = c.ClientKeygen({lbm_session_id: res.session_id, owner_id: "100001"});
        iterator.on("data", (data) => {
            console.log(data,'====================');
        });
    });
});
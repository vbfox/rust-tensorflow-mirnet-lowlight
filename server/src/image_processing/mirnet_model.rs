use anyhow::Context;
use anyhow::Result as AnyResult;
use std::path::Path;
use tensorflow::Graph;
use tensorflow::SavedModelBundle;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
use tensorflow::Tensor;
use tensorflow::DEFAULT_SERVING_SIGNATURE_DEF_KEY;

pub struct MirnetModel {
    graph: Graph,
    bundle: SavedModelBundle,
}

impl MirnetModel {
    pub fn new(model_dir: impl AsRef<Path>) -> AnyResult<MirnetModel> {
        let mut graph = Graph::new();
        let bundle =
            SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, model_dir)?;

        Ok(MirnetModel { graph, bundle })
    }

    pub fn run(&self, input: &Tensor<f32>) -> AnyResult<Tensor<f32>> {
        let signature = self
            .bundle
            .meta_graph_def()
            .get_signature(DEFAULT_SERVING_SIGNATURE_DEF_KEY)?;

        let (_, input_info) = signature.inputs().iter().next().context("No input found")?;
        let op_input = &self
            .graph
            .operation_by_name_required(&input_info.name().name)?;

        let (_, output_info) = signature
            .outputs()
            .iter()
            .next()
            .context("No output found")?;
        let op_output = &self
            .graph
            .operation_by_name_required(&output_info.name().name)?;

        let mut args = SessionRunArgs::new();
        args.add_feed(op_input, 0, input);
        let token_output = args.request_fetch(op_output, 0);

        self.bundle.session.run(&mut args)?;

        let output = args.fetch(token_output)?;
        Ok(output)
    }
}

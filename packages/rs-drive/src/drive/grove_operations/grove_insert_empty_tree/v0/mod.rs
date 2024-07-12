use crate::drive::grove_operations::push_drive_operation_result;
use crate::drive::Drive;
use crate::error::Error;
use crate::fee::op::LowLevelDriveOperation;
use grovedb::operations::insert::InsertOptions;
use grovedb::{Element, TransactionArg};
use grovedb_path::SubtreePath;
use platform_version::version::drive_versions::DriveVersion;

impl Drive {
    /// Pushes the `OperationCost` of inserting an empty tree in groveDB to `drive_operations`.
    pub(super) fn grove_insert_empty_tree_v0<B: AsRef<[u8]>>(
        &self,
        path: SubtreePath<'_, B>,
        key: &[u8],
        transaction: TransactionArg,
        options: Option<InsertOptions>,
        drive_operations: &mut Vec<LowLevelDriveOperation>,
        drive_version: &DriveVersion,
    ) -> Result<(), Error> {
        let cost_context = self.grove.insert(
            path,
            key,
            Element::empty_tree(),
            options,
            transaction,
            &drive_version.grove_version,
        );
        push_drive_operation_result(cost_context, drive_operations)
    }
}

const hash = require('../util/hash');
const serializer = require('../util/serializer');

const STPacketHeader = require('./STPacketHeader');

const EitherDapContractOrDapObjectsAllowedError = require('./errors/EitherDapContractOrDapObjectsAllowedError');

class STPacket {
  /**
   * @param {string} contractId
   */
  constructor(contractId) {
    this.setDapContractId(contractId);

    this.itemsMerkleRoot = null;
    this.itemsHash = null;

    this.objects = [];
    this.contracts = [];
  }

  // TODO Reuse code from STPacketHeader ?

  /**
   * Set Dap Contract ID
   *
   * @param {string} contractId
   */
  setDapContractId(contractId) {
    this.contractId = contractId;
  }

  /**
   * Get Dap Contract ID
   *
   * @return {string}
   */
  getDapContractId() {
    return this.contractId;
  }

  /**
   * Set items merkle root
   *
   * @param {string} itemsMerkleRoot
   */
  setItemsMerkleRoot(itemsMerkleRoot) {
    this.itemsMerkleRoot = itemsMerkleRoot;
  }

  /**
   * Get items merkle root
   *
   * @return {string}
   */
  getItemsMerkleRoot() {
    return this.itemsMerkleRoot;
  }

  /**
   * Set items hash
   *
   * @param {string} itemsHash
   */
  setItemsHash(itemsHash) {
    this.itemsHash = itemsHash;
  }

  /**
   * Get items hash
   *
   * @return {string}
   */
  getItemsHash() {
    return this.itemsHash;
  }

  /**
   * Set Dap Contract
   *
   * @param {DapContract} dapContract
   */
  setDapContract(dapContract) {
    if (this.objects.length) {
      throw new EitherDapContractOrDapObjectsAllowedError(this);
    }

    this.contracts = !dapContract ? [] : [dapContract];

    // TODO: set contract id
    // this.contractId = toHash(dapContract);

    return this;
  }

  /**
   * Get Dap Contract
   *
   * @return {DapContract|null}
   */
  getDapContract() {
    if (this.contracts.length) {
      return this.contracts[0];
    }

    return null;
  }

  /**
   * Set Dap Objects
   *
   * @param {DapObject[]} dapObjects
   */
  setDapObjects(dapObjects) {
    if (this.contracts.length) {
      throw new EitherDapContractOrDapObjectsAllowedError(this);
    }

    this.objects = dapObjects;
  }

  /**
   * Get Dap Objects
   *
   * @return {DapObject[]}
   */
  getDapObjects() {
    return this.objects;
  }

  /**
   * Add Dap Object
   *
   * @param {DapObject...} dapObjects
   */
  addDapObject(...dapObjects) {
    this.objects.push(...dapObjects);
  }

  /**
   * Create STPacketHeader with STPacket data
   *
   * @return STPacketHeader
   */
  extractHeader() {
    return STPacketHeader(
      this.getDapContractId(),
      this.getItemsMerkleRoot(),
      this.getItemsHash(),
    );
  }

  /**
   * Return ST Packet as plain object
   *
   * @return {{contractId: string, contracts: Object[], objects: Object[]}}
   */
  toJSON() {
    // TODO: Validate before to JSON ?
    return {
      contractId: this.getDapContractId(),
      itemsMerkleRoot: this.getItemsMerkleRoot(),
      itemsHash: this.getItemsHash(),
      contracts: this.contracts.map(dapContract => dapContract.toJSON()),
      objects: this.objects.map(dapObject => dapObject.toJSON()),
    };
  }

  /**
   * Return serialized ST Packet
   *
   * @return {Buffer}
   */
  serialize() {
    // TODO: Validate before serialization ?
    return serializer.encode(this.toJSON());
  }

  /**
   * Returns hex string with ST packet hash
   *
   * @return {string}
   */
  hash() {
    return hash(this.serialize());
  }
}

module.exports = STPacket;

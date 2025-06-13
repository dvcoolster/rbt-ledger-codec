#!/usr/bin/env python3
"""
FlowNet training script for RBT compression.

Usage:
    python train.py --data ./data/kodak --model out/model.yaml --epochs 250 --save-every 25
"""

import argparse
import os
from pathlib import Path
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import Dataset, DataLoader
from torchvision import transforms
from PIL import Image
import lightning as L
from tqdm import tqdm
import urllib.request
import zipfile


class KodakDataset(Dataset):
    """Kodak image dataset for training."""
    
    KODAK_URL = "http://r0k.us/graphics/kodak/kodak.zip"
    
    def __init__(self, root_dir, transform=None, download=True):
        self.root_dir = Path(root_dir)
        self.transform = transform
        
        if download and not self.root_dir.exists():
            self._download()
        
        # Find all PNG files
        self.image_paths = sorted(self.root_dir.glob("*.png"))
        if not self.image_paths:
            raise ValueError(f"No PNG files found in {root_dir}")
    
    def _download(self):
        """Download Kodak dataset if not present."""
        print(f"Downloading Kodak dataset to {self.root_dir}...")
        self.root_dir.mkdir(parents=True, exist_ok=True)
        
        zip_path = self.root_dir / "kodak.zip"
        urllib.request.urlretrieve(self.KODAK_URL, zip_path)
        
        with zipfile.ZipFile(zip_path, 'r') as zip_ref:
            zip_ref.extractall(self.root_dir)
        
        zip_path.unlink()  # Remove zip file
        print("Download complete!")
    
    def __len__(self):
        return len(self.image_paths)
    
    def __getitem__(self, idx):
        img_path = self.image_paths[idx]
        image = Image.open(img_path).convert('RGB')
        
        if self.transform:
            image = self.transform(image)
        
        return image, 0  # Return dummy label for compatibility


class FlowNetModel(nn.Module):
    """FlowNet model with invertible coupling blocks."""
    
    def __init__(self, levels=4, depth=4, channels=3):
        super().__init__()
        self.levels = levels
        self.depth = depth
        self.channels = channels
        
        # TODO: Implement coupling blocks
        # For now, just identity
        self.identity = nn.Identity()
    
    def forward(self, x, phase_tag=None):
        """Forward pass through the flow."""
        # TODO: Implement actual flow
        z = self.identity(x)
        log_det = torch.zeros(x.shape[0], device=x.device)
        return z, log_det
    
    def inverse(self, z, phase_tag=None):
        """Inverse pass through the flow."""
        # TODO: Implement actual inverse flow
        x = self.identity(z)
        return x


class FlowNetTrainer(L.LightningModule):
    """Lightning module for FlowNet training."""
    
    def __init__(self, model, learning_rate=1e-3):
        super().__init__()
        self.model = model
        self.learning_rate = learning_rate
        self.mse_loss = nn.MSELoss()
    
    def forward(self, x, phase_tag=None):
        return self.model(x, phase_tag)
    
    def training_step(self, batch, batch_idx):
        images, _ = batch
        
        # Forward pass
        z, log_det = self.model(images)
        
        # Prior loss (standard normal)
        prior_loss = 0.5 * torch.sum(z ** 2, dim=[1, 2, 3])
        
        # Negative log-likelihood
        nll = prior_loss - log_det
        loss = torch.mean(nll)
        
        self.log('train_loss', loss, prog_bar=True)
        return loss
    
    def configure_optimizers(self):
        return optim.Adam(self.model.parameters(), lr=self.learning_rate)


def main():
    parser = argparse.ArgumentParser(description='Train FlowNet model')
    parser.add_argument('--data', type=str, default='./data/kodak',
                        help='Path to Kodak dataset')
    parser.add_argument('--model', type=str, default='out/model.yaml',
                        help='Path to save model config')
    parser.add_argument('--epochs', type=int, default=250,
                        help='Number of training epochs')
    parser.add_argument('--save-every', type=int, default=25,
                        help='Save checkpoint every N epochs')
    parser.add_argument('--batch-size', type=int, default=16,
                        help='Batch size for training')
    parser.add_argument('--lr', type=float, default=1e-3,
                        help='Learning rate')
    parser.add_argument('--resume', type=str, default=None,
                        help='Resume from checkpoint')
    
    args = parser.parse_args()
    
    # Create output directories
    model_dir = Path(args.model).parent
    model_dir.mkdir(parents=True, exist_ok=True)
    checkpoint_dir = model_dir / 'checkpoints'
    checkpoint_dir.mkdir(exist_ok=True)
    
    # Data transforms
    transform = transforms.Compose([
        transforms.ToTensor(),
        transforms.Normalize(mean=[0.5, 0.5, 0.5], std=[0.5, 0.5, 0.5])
    ])
    
    # Dataset and dataloader
    dataset = KodakDataset(args.data, transform=transform)
    dataloader = DataLoader(dataset, batch_size=args.batch_size, 
                          shuffle=True, num_workers=4)
    
    # Model
    model = FlowNetModel()
    trainer_model = FlowNetTrainer(model, learning_rate=args.lr)
    
    # Callbacks
    checkpoint_callback = L.pytorch.callbacks.ModelCheckpoint(
        dirpath=checkpoint_dir,
        filename='epoch_{epoch:03d}',
        save_top_k=-1,
        every_n_epochs=args.save_every
    )
    
    # Trainer
    trainer = L.Trainer(
        max_epochs=args.epochs,
        callbacks=[checkpoint_callback],
        accelerator='auto',
        devices=1,
        logger=L.pytorch.loggers.TensorBoardLogger(model_dir, name='logs')
    )
    
    # Train
    trainer.fit(trainer_model, dataloader, ckpt_path=args.resume)
    
    # Save final checkpoint
    final_path = checkpoint_dir / 'epoch_last.pt'
    torch.save({
        'model_state_dict': model.state_dict(),
        'epoch': args.epochs,
        'args': vars(args)
    }, final_path)
    print(f"Training complete! Final model saved to {final_path}")


if __name__ == '__main__':
    main() 